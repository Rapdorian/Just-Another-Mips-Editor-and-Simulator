#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // if let Err(e) = run() {
    //     eprintln!("ERROR: {}", e);
    // }
    let app = simulator::App::default();
    let opts = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), opts);
}
