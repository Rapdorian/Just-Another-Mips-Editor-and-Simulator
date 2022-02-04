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
    Memory, RegisterFile,
};

#[derive(Default)]
pub struct App {
    script: String,
    console: String,
    show_watches: bool,
    watches: Vec<String>,
    mem: Memory,
    pc: u32,
    regs: RegisterFile,
    state: PipelineState,
    running: bool,
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
            show_watches,
            watches,
            mem,
            pc,
            regs,
            state,
            running,
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
                    for line in &lines {
                        match line {
                            Line::Instruction(ins) => {
                                for word in ins {
                                    raw_mem.push(word.asm(&labels));
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
                ui.add_sized(
                    ui.available_size(),
                    TextEdit::multiline(&mut "TODO: Console").code_editor(),
                );
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
                if ui.text_edit_singleline(console).lost_focus() {
                    if console.trim().len() != 0 {
                        watches.push(console.to_string());
                        *console = String::new();
                    }
                }
            });

        if *running {
            *state = pipeline::pipe_cycle(pc, regs, mem, state.clone());
            ctx.request_repaint();
        }
    }
    fn name(&self) -> &str {
        "Just Another Mips Editor and Simulator"
    }

    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
    }

    fn warm_up_enabled(&self) -> bool {
        false
    }

    fn save(&mut self, _storage: &mut dyn epi::Storage) {}

    fn on_exit(&mut self) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn max_size_points(&self) -> egui::Vec2 {
        // Some browsers get slow with huge WebGL canvases, so we limit the size:
        egui::Vec2::new(1024.0, 2048.0)
    }

    fn clear_color(&self) -> egui::Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }
}
