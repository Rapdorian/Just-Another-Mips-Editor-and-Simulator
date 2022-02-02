use eframe::{
    egui::{self, menu, TextEdit},
    epi,
};

#[derive(Default)]
pub struct App {
    script: String,
    console: String,
    show_watches: bool,
    watches: Vec<String>,
}

impl epi::App for App {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        let Self {
            script,
            console,
            show_watches,
            watches,
        } = self;

        egui::TopBottomPanel::top("Menu").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        *script = String::new();
                        ui.close_menu();
                    }
                    if ui.button("Open").clicked() {
                        println!("Filed Dialogs: Not Yet Implemented");
                        ui.close_menu();
                    }
                    if ui.button("Save").clicked() {
                        println!("Filed Dialogs: Not Yet Implemented");
                        ui.close_menu();
                    }
                });
            });

            menu::bar(ui, |ui| {
                if ui.button("▶").clicked() {
                    println!("TODO: Run");
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
    }
    fn name(&self) -> &str {
        "Just Another Mips Editor and Simulator"
    }
}
