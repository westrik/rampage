use crate::color::Color;
use crate::geometry::ray::Ray;
use crate::geometry::shape::Intersection;
use crate::material::*;
use crate::util::random::*;

pub struct Lambertian {
    pub albedo: Color,
}

impl Scatter for Lambertian {
    fn scatter(&self, _: &Ray, intersection: &Intersection) -> Option<Bounce> {
        let target = intersection.p + intersection.normal + random_vector_in_unit_sphere();
        Some(Bounce {
            attenuation: self.albedo.clone(),
            scattered: Ray {
                origin: intersection.p,
                direction: target - intersection.p,
            },
        })
    }
}
