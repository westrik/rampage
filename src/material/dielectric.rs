use crate::color::Color;
use crate::geometry::ray::Ray;
use crate::geometry::shape::Intersection;
use crate::material::{Bounce, Scatter};
use crate::Float;

pub struct Dielectric {
    pub refraction_index: Float,
}

impl Scatter for Dielectric {
    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<Bounce> {
        unimplemented!()
    }
}
