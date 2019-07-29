pub mod camera;
pub mod color;
pub mod geometry;
pub mod material;
pub mod output;

use crate::camera::build_camera;
use crate::geometry::vector::Vector;

use crate::color::Color;
use crate::geometry::ray::Ray;
use crate::geometry::scene::random_scene;
use crate::geometry::shape::Shape;
use crate::output::Image;
pub use core::f64 as float;

pub type Float = f64;

fn get_color(ray: &Ray, world: Vec<Shape>, depth: i32) -> Color {
    unimplemented!()
}

pub fn render_ball_scene() -> Image {
    let num_iterations = 100;
    let max_depth = 60;
    let width = 200; // TODO: 2000
    let height = 150; // TODO: 1500

    let from = Vector {
        x: 13.,
        y: 2.,
        z: 3.,
    };
    let to = Vector {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let distance_to_focus = 10.;
    let aperture = 0.1;

    let camera = build_camera(
        from,
        to,
        Vector {
            x: 0.,
            y: 1.,
            z: 0.,
        },
        Float::from(width) / Float::from(height),
        20.,
        aperture,
        distance_to_focus,
    );

    let world = random_scene();

    // TODO: add render loop

    /*
        for j in height, i in width:
            for iter in num_iterations:
                output_color += (shoot ray with jitter + calculate contribution)
            output_color /= num_iterations
            output_color = output_color.sqrt()
            add output color to pixel list
    */

    unimplemented!()
}
