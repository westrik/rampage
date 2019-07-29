use crate::geometry::ray::Ray;
use crate::geometry::sphere::*;
use crate::geometry::vector::Vector;
use crate::material::Material;
use crate::Float;

pub enum Shape {
    Sphere(Sphere),
}

pub struct Intersection<'a> {
    pub t: Float,
    pub p: Vector,
    pub normal: Vector,
    pub material: &'a Material,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<Intersection>;
}

impl Intersectable for Shape {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        match self {
            Shape::Sphere(sphere) => sphere.intersect(ray, t_min, t_max),
        }
    }
}

impl Intersectable for [Shape] {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let mut closest = t_max;
        let mut intersection = None;

        for shape in self {
            if let Some(i) = shape.intersect(ray, t_min, closest) {
                closest = i.t;
                intersection = Some(i)
            }
        }

        intersection
    }
}
