use crate::geometry::ray::*;
use crate::geometry::shape::*;
use crate::geometry::vector::*;
use crate::material::*;
use crate::Float;

pub struct Sphere {
    pub center: Vector,
    pub radius: Float,
    pub material: Material,
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        unimplemented!()
    }
}
