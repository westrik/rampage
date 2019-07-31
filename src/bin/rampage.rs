extern crate rampage;

use rampage::output::*;
use rampage::{render_scene, RenderConfig};

fn main() {
    let render_config = RenderConfig {
        num_threads: 8,
        num_iterations: 100,
        max_depth: 60,
        dimensions: (2000, 1500),
    };

    match render_scene("random_spheres", render_config) {
        Ok(image) => output_image(image, "output.png"),
        Err(msg) => println!("Failed because of {}", msg),
    }
}
