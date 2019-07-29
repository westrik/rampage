use crate::color::Color;
use crate::geometry::ray::Ray;
use crate::geometry::shape::Intersection;
use crate::material::{Bounce, Scatter};

pub struct Lambertian {
    pub albedo: Color,
}

impl Scatter for Lambertian {
    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<Bounce> {
        unimplemented!()
    }
}
