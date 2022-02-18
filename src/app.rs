use std::path::{Path, PathBuf};

use eframe::{
    egui::{self, menu, TextEdit},
    epi,
};
use futures::executor::block_on;
use rfd::AsyncFileDialog;

use crate::{
    parser::{self, compute_labels, model::Line},
    pipeline::{self, PipelineState},
    syscall::{resolve_syscall, Syscall},
    Memory, RegisterFile,
};

#[derive(Default)]
pub struct App {
    script: String,
    console: String,
    console_input: String,
    edit_watch: String,
    show_watches: bool,
    watches: Vec<String>,
    mem: Memory,
    pc: u32,
    regs: RegisterFile,
    state: PipelineState,
    running: bool,
    pending_syscall: Option<Syscall>,
}

async fn open_script() -> Option<String> {
    let file = AsyncFileDialog::new().pick_file().await;
    if let Some(file) = file {
        let bytes = file.read().await;
        Some(String::from_utf8_lossy(&bytes).to_string())
    } else {
        None
    }
}

impl epi::App for App {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        let Self {
            script,
            console,
            console_input,
            edit_watch,
            show_watches,
            watches,
            mem,
            pc,
            regs,
            state,
            running,
            pending_syscall,
        } = self;

        egui::TopBottomPanel::top("Menu").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        *script = String::new();
                        ui.close_menu();
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    if ui.button("Open").clicked() {
                        if let Some(file) = block_on(open_script()) {
                            *script = file;
                        }
                        ui.close_menu();
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    if ui.button("Save").clicked() {
                        println!("Filed Dialogs: Not Yet Implemented");
                        ui.close_menu();
                    }
                });
            });

            menu::bar(ui, |ui| {
                if ui.button("▶").clicked() {
                    // parse assembly
                    let lines = parser::parse_string(&script).unwrap();
                    let labels = compute_labels(&lines);

                    // for each line in the parsed assembly assemble that line and add the result to a vec
                    let mut raw_mem = vec![];
                    let mut assembler_pc = 0;
                    for line in &lines {
                        match line {
                            Line::Instruction(ins) => {
                                for word in ins {
                                    raw_mem.push(word.asm(&labels, assembler_pc));
                                    assembler_pc += 4;
                                }
                            }
                            Line::Label(_) => {}
                        }
                    }

                    // create our memory object
                    *mem = Memory::from_word_vec(raw_mem);

                    // reset machine
                    *pc = 0;
                    *regs = RegisterFile::default();
                    *state = PipelineState::default();
                    *running = true;
                    console.clear();
                }
                if ui.button("⬇").clicked() {
                    println!("TODO: Step into");
                }
                if ui.button("➡").clicked() {
                    println!("TODO: Step over");
                }
                if ui.button("⏏").clicked() {
                    println!("TODO: Step out");
                }
                ui.separator();
                if ui.button("🏷").clicked() {
                    *show_watches = !(*show_watches);
                }
            });
        });

        egui::TopBottomPanel::bottom("Console")
            .resizable(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(">");
                    if ui
                        .add_sized(
                            (ui.available_width(), 20.0),
                            TextEdit::singleline(console_input).code_editor(),
                        )
                        .lost_focus()
                    {
                        console.push_str(console_input);
                        console.push('\n');
                        if let Some(syscall) = pending_syscall {
                            if let Err(e) = resolve_syscall(regs, syscall, &console_input) {
                                console.push_str(&format!("\nERROR: {}\n", e));
                            }
                            console_input.clear();
                            *pending_syscall = None;
                        }
                    }
                });

                let mut console_in = console.clone();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add_sized(
                        ui.available_size(),
                        TextEdit::multiline(&mut console_in).code_editor(),
                    );
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_sized(
                ui.available_size(),
                TextEdit::multiline(script).code_editor(),
            );
        });

        let mut dummy_watch = String::from("$t42");

        egui::Window::new("Watches")
            .open(show_watches)
            .show(ctx, |ui| {
                ui.columns(2, |cols| {
                    for watch in watches.iter_mut() {
                        cols[0].text_edit_singleline(watch);
                        cols[1].label("???");
                    }
                });
                if ui.text_edit_singleline(edit_watch).lost_focus() {
                    if edit_watch.trim().len() != 0 {
                        watches.push(edit_watch.to_string());
                        *edit_watch = String::new();
                    }
                }
            });

        if *running {
            if let None = pending_syscall {
                let (new_state, syscall) = pipeline::pipe_cycle(pc, regs, mem, state.clone());
                *state = new_state;

                if let Some(syscall) = syscall {
                    match syscall {
                        Syscall::Print(out) => console.push_str(&out),
                        Syscall::Error(out) => console.push_str(&format!("\nERROR: {}\n", out)),
                        Syscall::Quit => *running = false,
                        Syscall::ReadInt => *pending_syscall = Some(syscall),
                    }
                }
                ctx.request_repaint();
            }
        }
    }
    fn name(&self) -> &str {
        "Just Another Mips Editor and Simulator"
    }
}
