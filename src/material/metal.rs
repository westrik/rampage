use crate::color::Color;
use crate::geometry::ray::Ray;
use crate::geometry::shape::Intersection;
use crate::geometry::vector::*;
use crate::material::*;
use crate::util::random::random_vector_in_unit_sphere;
use crate::Float;

pub struct Metal {
    pub albedo: Color,
    pub fuzziness: Float,
}

impl Reflect for Metal {
    fn reflect(&self, v: Vector, n: Vector) -> Vector {
        v - n.scale(2. * v.dot(n))
    }
}

impl Scatter for Metal {
    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<Bounce> {
        let reflected = self.reflect(ray.direction.to_unit(), intersection.normal);
        let scattered = Ray {
            origin: intersection.p,
            direction: reflected + random_vector_in_unit_sphere().scale(self.fuzziness),
        };
        if scattered.direction.dot(intersection.normal) > 0. {
            Some(Bounce {
                attenuation: self.albedo.clone(),
                scattered,
            })
        } else {
            None
        }
    }
}
