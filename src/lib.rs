pub mod camera;
pub mod color;
pub mod geometry;
pub mod material;
pub mod output;
pub mod util;

use crate::camera::*;
use crate::color::*;
use crate::geometry::vector::*;
use crate::material::*;

use crate::geometry::ray::Ray;
use crate::geometry::scene::random_scene;
use crate::geometry::shape::*;
use crate::output::Image;
pub use core::f64 as float;
use rand::prelude::*;

pub type Float = f64;

fn get_color(ray: &Ray, world: &[Shape], current_depth: i32, max_depth: i32) -> Color {
    if let Some(intersection) = world.intersect(ray, 0.001, float::MAX) {
        if let Some(bounce) = intersection.material.scatter(ray, &intersection) {
            if current_depth < max_depth {
                return bounce.attenuation
                    * get_color(&bounce.scattered, world, current_depth + 1, max_depth);
            }
        }
        BLACK
    } else {
        let unit_direction = ray.direction.to_unit();
        let t = 0.5 * unit_direction.y + 1.;
        Color {
            r: (1. - t),
            g: (1. - t),
            b: (1. - t),
        } + Color {
            r: t * 0.5,
            g: t * 0.7,
            b: t,
        }
    }
}

pub fn render_ball_scene() -> Image {
    /*
    TODO:
    - add parallelism
    - refactoring + add more tests
    */

    let num_iterations = 10;
    let max_depth = 60;
    let width: u32 = 200; // TODO: 2000
    let height: u32 = 150; // TODO: 1500

    let camera = build_camera(
        Vector {
            x: 13.,
            y: 2.,
            z: 3.,
        },
        Vector {
            x: 0.,
            y: 0.,
            z: 0.,
        },
        Vector {
            x: 0.,
            y: 1.,
            z: 0.,
        },
        20.,
        Float::from(width) / Float::from(height),
        0.1,
        10.,
    );

    let mut pixels = vec![BLACK; (width * height) as usize];

    let world = random_scene();
    let num_iterations_f = Float::from(num_iterations);
    let mut rng = rand::thread_rng();

    for i in 0..width {
        for j in 0..height {
            let mut pixel = BLACK;
            for _ in 0..num_iterations {
                let u = (Float::from(i) + rng.gen::<Float>()) / Float::from(width);
                let v = (Float::from(j) + rng.gen::<Float>()) / Float::from(height);
                let ray = camera.get_ray(u, v);
                pixel += get_color(&ray, &world, 0, max_depth);
            }
            pixel = Color {
                r: pixel.r / num_iterations_f,
                g: pixel.g / num_iterations_f,
                b: pixel.b / num_iterations_f,
            };
            // Gamma adjustment.
            pixel = Color {
                r: pixel.r.sqrt(),
                g: pixel.g.sqrt(),
                b: pixel.b.sqrt(),
            };
            pixels[((height - j - 1) * width + i) as usize] = pixel;
        }
    }

    Image {
        pixels,
        height,
        width,
    }
}
