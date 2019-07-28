extern crate rampage;

use rampage::color::RgbColor;
use rampage::output::{output_image, Image};

fn main() {
    let width = 500;
    let height = 500;
    let pixels: Vec<_> = (0..(width * height))
        .map(|_| RgbColor {
            red: 255,
            blue: 255,
            green: 0,
        })
        .collect();
    let image = Image {
        width,
        height,
        pixels,
    };

    output_image(&image, "output.png");
}
