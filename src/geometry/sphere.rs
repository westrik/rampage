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
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0. {
            let t = (-b - discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                return Some(Intersection {
                    t,
                    p: ray.point(t),
                    normal: (ray.point(t) - self.center).scale(1. / self.radius),
                    material: &self.material,
                });
            }
        }

        None
    }
}
