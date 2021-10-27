#![warn(clippy::all, rust_2018_idioms)]

use othello::othello;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // let app = othello::TemplateApp::default();
    let app = othello::OthelloApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
