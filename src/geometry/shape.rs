use crate::geometry::ray::Ray;
use crate::geometry::sphere::Sphere;
use crate::geometry::vector::Vector;
use crate::material::Material;
use crate::Float;

pub enum Shape {
    Sphere(Sphere),
}

pub struct Intersection {
    pub t: Float,
    pub p: Vector,
    pub normal: Vector,
    pub material: Material,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<Intersection>;
}

impl Intersectable for &[Shape] {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        unimplemented!()
    }
}
