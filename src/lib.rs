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
use std::sync::{Arc, Mutex};
use std::thread;

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

// TODO: put all of this data in a SceneConfig struct.
#[allow(clippy::too_many_arguments)]
fn render_pixel(
    scene: &[Shape],
    camera: &Camera,
    i: u32,
    j: u32,
    width: u32,
    height: u32,
    num_iterations: u32,
    max_depth: i32,
) -> Color {
    let num_iterations_f = Float::from(num_iterations);
    let mut rng = rand::thread_rng();

    let mut pixel = BLACK;
    for _ in 0..num_iterations {
        let u = (Float::from(i) + rng.gen::<Float>()) / Float::from(width);
        let v = (Float::from(j) + rng.gen::<Float>()) / Float::from(height);
        let ray = camera.get_ray(u, v);
        pixel += get_color(&ray, scene, 0, max_depth);
    }
    pixel = Color {
        r: pixel.r / num_iterations_f,
        g: pixel.g / num_iterations_f,
        b: pixel.b / num_iterations_f,
    };
    // Return pixel with gamma adjustment.
    Color {
        r: pixel.r.sqrt(),
        g: pixel.g.sqrt(),
        b: pixel.b.sqrt(),
    }
}

pub fn render_ball_scene() -> Image {
    /*
    TODO:
    - refactoring + add more tests
    */
    let num_iterations = 100;
    let max_depth = 60;
    let width: u32 = 2000;
    let height: u32 = 1500;

    let pixels = Arc::new(Mutex::new(vec![BLACK; (width * height) as usize]));
    let mut thread_handles = vec![];

    let world = Arc::new(random_scene());

    let num_threads = 12;
    for thread_id in 0..num_threads {
        let pixels = Arc::clone(&pixels);
        let world = Arc::clone(&world);
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
        let handle = thread::spawn(move || {
            let world = world;
            let cols_per_thread = width / num_threads;
            let start = thread_id * cols_per_thread;
            let end = if thread_id == num_threads - 1 {
                width
            } else {
                (thread_id + 1) * cols_per_thread
            };

            for i in start..end {
                for j in 0..height {
                    pixels.lock().unwrap()[((height - j - 1) * width + i) as usize] = render_pixel(
                        &world,
                        &camera,
                        i,
                        j,
                        width,
                        height,
                        num_iterations,
                        max_depth,
                    );
                }
            }
        });
        thread_handles.push(handle);
    }

    for handle in thread_handles {
        handle.join().unwrap()
    }

    Image {
        pixels: Arc::clone(&pixels),
        height,
        width,
    }
}
