# Image Converter

A fast and efficient image converter application built with Rust, featuring a simple GUI interface.

## Features

- Convert images between PNG, JPG, and WebP formats
- Adjustable compression levels (1-100%)
- Batch processing of multiple images
- Parallel processing for faster conversion
- Simple and intuitive GUI interface

## Requirements

- Rust 2021 edition or later
- Cargo package manager

## Installation

1. Clone this repository
2. Navigate to the project directory
3. Build the project:
```bash
cargo build --release
```

## Usage

1. Run the application:
```bash
cargo run --release
```

2. Use the GUI to:
   - Select input image files
   - Choose output format
   - Adjust compression level
   - Click "Convert Images" to start the conversion

## Technical Details

- Built using the `eframe` (egui) framework for the GUI
- Uses the `image` crate for image processing
- Implements parallel processing with `rayon`
- File dialog functionality provided by `rfd`

## License

MIT License