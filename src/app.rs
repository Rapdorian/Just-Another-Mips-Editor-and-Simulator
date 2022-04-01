use std::fs::read_to_string;

use eframe::{
    egui::{self, menu, ScrollArea},
    epi,
};

use rfd::FileDialog;

use crate::{Machine, Register};

use self::{
    console::Console,
    editor::Editor,
    pipeline_view::PipelineView,
    run_menu::RunMenu,
    watches::{Watch, WatchList},
};

mod console;
mod editor;
mod pipeline_view;
mod run_menu;
mod watches;

#[derive(Default)]
pub struct App {
    machine: Machine,
    script: String,
    console: Console,
    show_watches: bool,
    show_stack: bool,
    show_pipeline: bool,
    show_regs: bool,
    regs_hex: bool,
    stack_hex: bool,
    watches: Vec<Watch>,
    running: bool,
}

fn open_script() -> Option<String> {
    let file = FileDialog::new().set_directory(".").pick_file();
    if let Some(path) = file {
        let file = read_to_string(path).unwrap();
        Some(file)
    } else {
        None
    }
}

impl epi::App for App {
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        let Self {
            machine,
            script,
            console,
            show_watches,
            show_stack,
            show_pipeline,
            show_regs,
            regs_hex,
            stack_hex,
            watches,
            running,
        } = self;

        // Draw the watches in their own window, draw it first so the window is not constrained to
        // a specific part of the screen
        egui::Window::new("Watches")
            .open(show_watches)
            .show(ctx, |ui| ui.add(WatchList::new(watches, machine)));

        // draw the menu bars
        egui::TopBottomPanel::top("Menu").show(ctx, |ui| {
            // draw main menu bar (not much to put here yet)
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        *script = String::new();
                        ui.close_menu();
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    if ui.button("Open").clicked() {
                        if let Some(file) = open_script() {
                            *script = file;
                        }
                        ui.close_menu();
                    }

                    // #[cfg(not(target_arch = "wasm32"))]
                    // if ui.button("Save").clicked() {
                    //     println!("Filed Dialogs: Not Yet Implemented");
                    //     ui.close_menu();
                    // }
                });
                ui.menu_button("View", |ui| {
                    if ui.button("Watches").clicked() {
                        *show_watches = true;
                        ui.close_menu();
                    }
                    if ui.button("Toggle Stack View").clicked() {
                        *show_stack = !*show_stack;
                        ui.close_menu();
                    }
                    if ui.button("Toggle Pipeline View").clicked() {
                        *show_pipeline = !*show_pipeline;
                        ui.close_menu();
                    }
                    if ui.button("Toggle Register View").clicked() {
                        *show_regs = !*show_regs;
                        ui.close_menu();
                    }
                });
            });

            // draw toolbar
            menu::bar(ui, |ui| {
                // add run menu
                ui.add(RunMenu::new(ctx, machine, running, script, console));

                // toggle watches
                ui.separator();
                if ui.button("🏷").clicked() {
                    *show_watches = !(*show_watches);
                }
            });
        });

        // Draw the console in a bottom panel
        egui::TopBottomPanel::bottom("Console")
            .resizable(true)
            .show(ctx, |ui| {
                if ui.add(console.view()).changed() {
                    // if we get input from the console we want to try to use that to resolve a
                    // system call
                    if let Some(input) = console.input() {
                        // if resolving the system call failed send an error to the console
                        if let Err(e) = machine.resolve_input(input) {
                            console.error(&e.to_string());
                            // and stop the program
                            *running = false;
                        }
                    }
                }
            });

        if *show_stack {
            // Display the stack in a sidebar
            egui::SidePanel::right("stack").show(ctx, |ui| {
                ui.heading("Stack");
                ui.horizontal(|ui| {
                    ui.radio_value(stack_hex, false, "Dec");
                    ui.radio_value(stack_hex, true, "Hex");
                });
                for (addr, item) in machine.stack().iter().rev() {
                    ui.horizontal(|ui| {
                        ui.label(format!("(0x{addr:X}): "));
                        let val = if *stack_hex {
                            format!("0x{item:X}")
                        } else {
                            format!("{item}")
                        };
                        ui.label(val);
                    });
                }
            });
        }

        if *show_regs {
            // Display the registers in a sidebar
            egui::SidePanel::right("registers").show(ctx, |ui| {
                ui.heading("Registers");
                ui.horizontal(|ui| {
                    ui.radio_value(regs_hex, false, "Dec");
                    ui.radio_value(regs_hex, true, "Hex");
                });
                ScrollArea::vertical().show(ui, |ui| {
                    for r in 0..32 {
                        let r = Register::from(r);
                        ui.horizontal(|ui| {
                            let name = r.name();
                            let val = machine.register(r);
                            ui.label(format!("{name}: "));
                            let val = if *regs_hex {
                                format!("0x{val:X}")
                            } else {
                                format!("{val}")
                            };
                            ui.label(format!("{val}"));
                        });
                    }
                });
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if *show_pipeline {
                ui.add(PipelineView::new(machine));
            }
            ui.add(Editor::new(script, &machine.current_line()))
        });
    }
    fn name(&self) -> &str {
        "Just Another Mips Editor and Simulator"
    }
}
