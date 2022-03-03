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
    Machine, Memory, RegisterFile,
};

use self::{
    console::Console,
    editor::Editor,
    run_menu::RunMenu,
    watches::{Watch, WatchList},
};

mod console;
mod editor;
mod run_menu;
mod watches;

#[derive(Default)]
pub struct App {
    machine: Machine,
    script: String,
    console: Console,
    show_watches: bool,
    watches: Vec<Watch>,
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
            machine,
            script,
            console,
            show_watches,
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
                        if let Some(file) = block_on(open_script()) {
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

        egui::CentralPanel::default().show(ctx, |ui| ui.add(Editor::new(script)));
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
