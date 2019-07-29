use crate::geometry::ray::Ray;
use crate::geometry::vector::*;
use crate::{float, Float};
use rand::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Camera {
    pub origin: Vector,
    pub lower_left: Vector,
    pub horizontal: Vector,
    pub vertical: Vector,
    pub u: Vector,
    pub v: Vector,
    pub w: Vector,
    pub lens_radius: Float,
}

fn get_random_vector_in_unit_disk() -> Vector {
    let mut rng = rand::thread_rng();
    let mut vector;
    loop {
        vector = Vector {
            x: 2. * rng.gen::<Float>() - 1.,
            y: 2. * rng.gen::<Float>() - 1.,
            z: 0.,
        };
        if vector.dot(vector) < 1. {
            break;
        }
    }
    vector
}

pub trait RayGenerator {
    fn get_ray(&self, s: Float, t: Float) -> Ray;
}

impl RayGenerator for Camera {
    fn get_ray(&self, s: Float, t: Float) -> Ray {
        let rd = get_random_vector_in_unit_disk().scale(self.lens_radius);
        let offset = self.u.scale(rd.x) + self.v.scale(rd.y);

        Ray {
            origin: self.origin + offset,
            direction: self.lower_left
                + self.horizontal.scale(s)
                + (self.vertical - self.origin - offset).scale(t),
        }
    }
}

// TODO: add documentation.
pub fn build_camera(
    origin: Vector,
    to: Vector,
    vup: Vector,
    vertical_fov: Float,
    aspect: Float,
    aperture: Float,
    focus_distance: Float,
) -> Camera {
    let lens_radius = aperture / 2.;
    let theta = vertical_fov * float::consts::PI / 180.;
    let half_height = (theta / 2.).tan();
    let half_width = aspect * half_height;

    let w = (origin - to).to_unit();
    let u = vup.cross(w).to_unit();
    let v = w.cross(u);

    Camera {
        origin,
        lower_left: origin
            - u.scale(half_width * focus_distance)
            - v.scale(half_height * focus_distance)
            - w.scale(focus_distance),
        horizontal: u.scale(2. * half_width * focus_distance),
        vertical: v.scale(2. * half_height * focus_distance),
        u,
        v,
        w,
        lens_radius,
    }
}

#[cfg(test)]
mod test_camera {
    use super::*;
    use crate::geometry::vector::test_vectors::*;

    #[test]
    fn test_camera_setup() {
        // TODO: add test that derives individual result values
    }
}
