use eframe::egui;
use image::ImageFormat;
use crate::types::ImageConverterApp;
use crate::converter::convert_images;

impl eframe::App for ImageConverterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Image Converter");
            ui.horizontal(|ui| {
                ui.label("Selected files:");
                ui.label(format!("{} files", self.input_files.len()));
            });

            if ui.button("Select Files").clicked() {
                if let Some(files) = rfd::FileDialog::new()
                    .add_filter("Images", &["png", "jpg", "jpeg", "webp"])
                    .pick_files()
                {
                    self.input_files = files;
                }
            }

            ui.horizontal(|ui| {
                ui.label("Output Format:");
                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", self.output_format))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.output_format, ImageFormat::Jpeg, "JPEG");
                        ui.selectable_value(&mut self.output_format, ImageFormat::Png, "PNG");
                        ui.selectable_value(&mut self.output_format, ImageFormat::WebP, "WebP");
                    });
            });

            ui.horizontal(|ui| {
                ui.label("Compression Level:");
                ui.add(egui::Slider::new(&mut self.compression_level, 1..=100));
                ui.label(format!("{}%", self.compression_level));
            });

            if ui.button("Convert Images").clicked() && !self.input_files.is_empty() {
                self.status = "Converting images...".to_string();
                self.status = convert_images(
                    &self.input_files,
                    self.output_format,
                    self.compression_level,
                );
            }

            ui.label(&self.status);
        });
    }
} 