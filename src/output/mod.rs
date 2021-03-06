use crate::color::Color;
use image::png::PNGEncoder;
use image::ColorType;
use std::fs::File;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub struct Image {
    pub pixels: Arc<Mutex<Vec<Color>>>,
    pub height: u32,
    pub width: u32,
}

pub fn output_image(image: Image, path: &str) {
    let output_file = File::create(Path::new(path)).expect("couldn't open file");

    let pixels = image.pixels.lock().unwrap();

    let mut raw_pixels = Vec::with_capacity(pixels.len() * 3);
    for pixel in pixels.iter() {
        raw_pixels.push((pixel.r * 255.99) as u8);
        raw_pixels.push((pixel.g * 255.99) as u8);
        raw_pixels.push((pixel.b * 255.99) as u8);
    }

    PNGEncoder::new(output_file)
        .encode(&raw_pixels, image.width, image.height, ColorType::RGB(8))
        .expect("couldn't output png")
}
