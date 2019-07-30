extern crate rampage;

use rampage::output::*;
use rampage::render_ball_scene;

fn main() {
    output_image(render_ball_scene(), "output.png");
}
