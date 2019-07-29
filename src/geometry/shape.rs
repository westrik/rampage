use crate::geometry::ray::Ray;
use crate::geometry::sphere::Sphere;
use crate::geometry::vector::Vector;
use crate::Float;

pub enum Shape {
    Sphere(Sphere),
}

pub struct Intersection {
    pub t: Float,
    pub p: Vector,
    pub normal: Vector,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray, t_min: Float, t_max: Float, intersection: &Intersection)
        -> bool;
}

impl Intersectable for Vec<Shape> {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64, intersection: &Intersection) -> bool {
        unimplemented!()
    }
}
