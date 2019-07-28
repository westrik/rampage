use crate::color::RgbColor;
use image::png::PNGEncoder;
use image::ColorType;
use std::fs::File;
use std::path::Path;

pub struct Image {
    pub pixels: Vec<RgbColor>,
    pub height: u32,
    pub width: u32,
}

pub fn output_image(image: &Image, path: &str) {
    let output_file = File::create(Path::new(path)).expect("couldn't open file");

    let mut raw_pixels = Vec::with_capacity(image.pixels.len() * 3);
    for pixel in &image.pixels {
        raw_pixels.push(pixel.red);
        raw_pixels.push(pixel.green);
        raw_pixels.push(pixel.blue);
    }

    PNGEncoder::new(output_file)
        .encode(&raw_pixels, image.width, image.height, ColorType::RGB(8))
        .expect("couldn't output png")
}
