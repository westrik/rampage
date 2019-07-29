pub mod camera;
pub mod color;
pub mod geometry;
pub mod material;
pub mod output;
pub mod util;

use crate::camera::*;
use crate::color::Color;
use crate::geometry::vector::*;
use crate::material::*;

use crate::geometry::ray::Ray;
use crate::geometry::scene::random_scene;
use crate::geometry::shape::*;
use crate::output::Image;
pub use core::f64 as float;
use rand::prelude::*;

pub type Float = f64;

fn get_color(ray: &Ray, world: &[Shape], depth: i32) -> Color {
    let intersection = world.intersect(ray, 0.001, float::MAX);
    match intersection {
        Some(i) => {
            let bounce = i.material.scatter(ray, &i);
            match bounce {
                Some(b) => b.attenuation * get_color(&b.scattered, world, depth + 1),
                None => Color {
                    r: 0.,
                    g: 0.,
                    b: 0.,
                },
            }
        }
        None => {
            let unit_direction = ray.direction.to_unit();
            let t = 0.5 * unit_direction.y + 1.;
            Color {
                r: 1. - t,
                g: 1. - t,
                b: 1. - t,
            } * Color {
                r: t * 0.5,
                g: t * 0.7,
                b: t,
            }
        }
    }
}

pub fn render_ball_scene() -> Image {
    let num_iterations = 100;
    let max_depth = 60;
    let width: u32 = 200; // TODO: 2000
    let height: u32 = 150; // TODO: 1500

    /*
    TODO:
    - implement Dielectric & Metal materials
    - add parallelism
    - finish random scene generation
    - [verify] fix bug where everything looks really dark?
    - refactoring + add more tests
    */

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
    let focus_distance = 10.;
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
        focus_distance,
    );

    let mut pixels = vec![
        Color {
            r: 0.,
            g: 0.,
            b: 0.
        };
        (width * height) as usize
    ];

    let world = random_scene();
    let num_iterations_f = Float::from(num_iterations);
    let mut rng = rand::thread_rng();

    for j in 0..height {
        for i in 0..width {
            let mut pixel = Color {
                r: 0.,
                g: 0.,
                b: 0.,
            };
            for iteration in 0..num_iterations {
                let u = (Float::from(i) + rng.gen::<Float>()) / Float::from(width);
                let v = (Float::from(j) + rng.gen::<Float>()) / Float::from(height);
                let ray = camera.get_ray(u, v);
                pixel += get_color(&ray, &world, 0);
            }
            pixel = Color {
                r: pixel.r / num_iterations_f,
                g: pixel.g / num_iterations_f,
                b: pixel.b / num_iterations_f,
            };
            pixel = Color {
                r: pixel.r.sqrt(),
                g: pixel.g.sqrt(),
                b: pixel.b.sqrt(),
            };
            pixels[(j * width + i) as usize] = pixel;
        }
    }

    Image {
        pixels,
        height,
        width,
    }
}
