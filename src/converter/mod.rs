use std::path::PathBuf;
use std::fs;
use image::{ImageFormat, ImageOutputFormat};
use rayon::prelude::*;

pub fn convert_images(
    input_files: &[PathBuf],
    output_format: ImageFormat,
    compression_level: u8,
) -> String {
    let compression = compression_level;
    let format = output_format;

    input_files.par_iter().for_each(|input_path| {
        if let Ok(img) = image::open(input_path) {
            let output_path = input_path.with_extension(format.extension().unwrap_or("jpg"));
            let mut output = Vec::new();
            
            let output_format = match format {
                ImageFormat::Jpeg => ImageOutputFormat::Jpeg(compression),
                ImageFormat::Png => ImageOutputFormat::Png,
                ImageFormat::WebP => ImageOutputFormat::WebP,
                _ => ImageOutputFormat::Jpeg(compression),
            };

            if let Ok(_) = img.write_with_encoder(
                image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output, compression)
            ) {
                if let Err(e) = fs::write(&output_path, output) {
                    eprintln!("Error writing {}: {}", output_path.display(), e);
                }
            }
        }
    });

    "Conversion complete!".to_string()
} 