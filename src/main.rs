#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // if let Err(e) = run() {
    //     eprintln!("ERROR: {}", e);
    // }

    use eframe::egui::Visuals;
    let app = simulator::App::default();
    let opts = eframe::NativeOptions::default();
    eframe::run_native(
        "Just Another Mips Editor and Simulator",
        opts,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(Visuals::dark());
            Box::new(app)
        }),
    );
}
