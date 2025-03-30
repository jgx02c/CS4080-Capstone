use std::path::PathBuf;
use image::ImageFormat;

pub struct ImageConverterApp {
    pub input_files: Vec<PathBuf>,
    pub output_format: ImageFormat,
    pub compression_level: u8,
    pub status: String,
}

impl Default for ImageConverterApp {
    fn default() -> Self {
        Self {
            input_files: Vec::new(),
            output_format: ImageFormat::Jpeg,
            compression_level: 80,
            status: "Ready".to_string(),
        }
    }
} 