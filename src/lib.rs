pub mod camera;
pub mod color;
pub mod geometry;
pub mod materials;
pub mod output;
pub mod util;

use crate::camera::*;
use crate::color::*;
use crate::geometry::vector::*;
use crate::materials::*;

use crate::geometry::ray::Ray;
use crate::geometry::scene::{random_spheres_scene, Scene};
use crate::geometry::shape::*;
use crate::output::Image;
use console::{style, Emoji};
pub use core::f64 as float;
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use rand::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

pub type Float = f64;

#[derive(Clone, Copy)]
pub struct RenderConfig {
    pub num_threads: u32,
    pub num_iterations: u32,
    pub max_depth: i32,
    pub dimensions: (u32, u32),
}

static ANGRY_POO: Emoji<'_, '_> = Emoji("୧༼ಠ益ಠ༽୨", "RAMPAGE");

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

fn render_at_coordinates(i: u32, j: u32, sc: &Scene, rc: RenderConfig) -> Color {
    let num_iterations_f = Float::from(rc.num_iterations);
    let (width, height) = rc.dimensions;
    let mut rng = rand::thread_rng();

    let mut pixel = BLACK;
    for _ in 0..rc.num_iterations {
        let u = (Float::from(i) + rng.gen::<Float>()) / Float::from(width);
        let v = (Float::from(j) + rng.gen::<Float>()) / Float::from(height);
        let ray = sc.camera.get_ray(u, v);
        pixel += get_color(&ray, &sc.shapes, 0, rc.max_depth);
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

pub fn render_scene(scene_name: &str, render_config: RenderConfig) -> Result<Image, &'static str> {
    match scene_name {
        "random_spheres" => Ok(render_using_threads(
            render_config,
            random_spheres_scene(render_config.dimensions),
        )),
        _ => Err("invalid scene_name"),
    }
}

pub fn render_using_threads(render_config: RenderConfig, scene: Scene) -> Image {
    let (width, height) = render_config.dimensions;
    let num_threads = render_config.num_threads;
    let mut thread_handles = vec![];

    let pixels = Arc::new(Mutex::new(vec![BLACK; (width * height) as usize]));
    let scene = Arc::new(scene);

    let m = MultiProgress::new();
    let sty = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .progress_chars("##-");
    let started = Instant::now();

    for thread_id in 0..num_threads {
        let pixels = Arc::clone(&pixels);
        let scene = Arc::clone(&scene);
        let cols_per_thread = width / num_threads;
        let start_col = thread_id * cols_per_thread;
        let end_col = if thread_id == num_threads - 1 {
            width
        } else {
            (thread_id + 1) * cols_per_thread
        };
        let pb = m.add(ProgressBar::new(u64::from(end_col - start_col)));
        pb.set_style(sty.clone());

        let handle = thread::spawn(move || {
            for i in start_col..end_col {
                pb.set_message(&format!("rendering column #{}", i));
                pb.inc(1);
                for j in 0..height {
                    pixels.lock().unwrap()[((height - j - 1) * width + i) as usize] =
                        render_at_coordinates(i, j, &scene, render_config);
                }
            }
            pb.finish_with_message("done");
        });

        thread_handles.push(handle);
    }
    m.join_and_clear().unwrap();
    println!(
        "[{}] Done in {}",
        style(ANGRY_POO).red(),
        HumanDuration(started.elapsed())
    );

    Image {
        pixels: Arc::clone(&pixels),
        height,
        width,
    }
}
