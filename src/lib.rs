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
use crate::rampage::RenderConfig;
pub use core::f64 as float;

pub type Float = f64;

pub mod rampage {
    use super::*;

    #[derive(Clone, Copy)]
    pub struct RenderConfig {
        pub num_threads: u32,
        pub num_iterations: u32,
        pub max_depth: i32,
        pub dimensions: (u32, u32),
    }

    pub fn render_scene(
        scene_name: &str,
        render_config: RenderConfig,
    ) -> Result<Image, &'static str> {
        match scene_name {
            "random_spheres" => Ok(render_using_threads(
                render_config,
                random_spheres_scene(render_config.dimensions),
            )),
            _ => Err("invalid scene_name"),
        }
    }
}

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
    let (width, height) = rc.dimensions;

    let u = Float::from(i) / Float::from(width);
    let v = Float::from(j) / Float::from(height);
    let ray = sc.camera.get_ray(u, v);
    let pixel = get_color(&ray, &sc.shapes, 0, rc.max_depth);
    // Return pixel with gamma adjustment.
    Color {
        r: pixel.r.sqrt(),
        g: pixel.g.sqrt(),
        b: pixel.b.sqrt(),
    }
}

pub fn render_using_threads(render_config: RenderConfig, scene: Scene) -> Image {
    let (width, height) = render_config.dimensions;

    let mut pixels = vec![BLACK; (width * height) as usize];

    for i in 0..width {
        for j in 0..height {
            pixels[((height - j - 1) * width + i) as usize] =
                render_at_coordinates(i, j, &scene, render_config);
        }
    }

    Image {
        pixels,
        height,
        width,
    }
}
