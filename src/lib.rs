pub mod camera;
pub mod color;
pub mod geometry;
pub mod material;
pub mod output;

use image::png::PNGEncoder as PngEncoder;
use image::ColorType;
use std::fs::File;
use std::path::Path;

pub fn output_image() {
    let width = 500 as u32;
    let height = 500 as u32;
    let pixels: Vec<_> = (0..(width * height * 3)).map(|_| 0).collect::<Vec<_>>();

    let path = Path::new("output.png");
    PngEncoder::new(File::create(path).expect("could not open file"))
        .encode(&pixels, width, height, ColorType::RGB(8))
        .expect("could not output png")
}
