extern crate rampage;

use rampage::output::*;
use rampage::rampage::{render_scene, RenderConfig};

fn main() {
    let render_config = RenderConfig {
        num_threads: 1,
        num_iterations: 2,
        max_depth: 5,
        dimensions: (100, 80),
    };

    match render_scene("random_spheres", render_config) {
        Ok(image) => output_image(image, "output.png"),
        Err(msg) => println!("Failed because of {}", msg),
    }
}
