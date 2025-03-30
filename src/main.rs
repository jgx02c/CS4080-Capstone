mod types;
mod converter;
mod gui;

use types::ImageConverterApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(400.0, 300.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Image Converter",
        options,
        Box::new(|_cc| Box::new(ImageConverterApp::default())),
    )
}