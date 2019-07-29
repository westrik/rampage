use crate::color::Color;
use crate::geometry::ray::Ray;
use crate::geometry::shape::Intersection;
use crate::geometry::vector::*;
use crate::material::*;
use crate::util::random::*;
use rand::prelude::*;

pub struct Lambertian {
    pub albedo: Color,
}

impl Scatter for Lambertian {
    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<Bounce> {
        let target = intersection.p + intersection.normal + get_random_vector_in_unit_sphere();
        let scattered = Ray {
            origin: intersection.p,
            direction: target - intersection.p,
        };

        Some(Bounce {
            attenuation: self.albedo.clone(),
            scattered,
        })
    }
}
