pub mod dielectric;
pub mod lambertian;
pub mod metal;

use crate::color::Color;
use crate::geometry::ray::Ray;
use crate::geometry::shape::Intersection;
use crate::geometry::vector::Vector;
use crate::Float;
use dielectric::Dielectric;
use lambertian::Lambertian;
use metal::Metal;

pub enum Material {
    Dielectric(Dielectric),
    Lambertian(Lambertian),
    Metal(Metal),
}

pub struct Bounce {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Scatter {
    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<Bounce>;
}
impl Scatter for Material {
    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<Bounce> {
        match self {
            Material::Dielectric(material) => material.scatter(ray, intersection),
            Material::Lambertian(material) => material.scatter(ray, intersection),
            Material::Metal(material) => material.scatter(ray, intersection),
        }
    }
}

pub trait Reflect {
    fn reflect(&self, v: Vector, n: Vector) -> Vector;
}
impl Reflect for Material {
    fn reflect(&self, v: Vector, n: Vector) -> Vector {
        unimplemented!()
    }
}

pub trait Refract {
    fn refract(&self, v: Vector, n: Vector, ni_over_nt: Float, refracted: Vector) -> Vector;
}
impl Refract for Material {
    fn refract(&self, v: Vector, n: Vector, ni_over_nt: f64, refracted: Vector) -> Vector {
        unimplemented!()
    }
}
