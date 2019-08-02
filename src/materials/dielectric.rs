use crate::color::*;
use crate::geometry::ray::Ray;
use crate::geometry::shape::Intersection;
use crate::geometry::vector::*;
use crate::materials::*;
use crate::Float;

pub struct Dielectric {
    pub refraction_index: Float,
}

impl Refract for Dielectric {
    fn refract(&self, v: Vector, n: Vector, ni_over_nt: f64) -> Option<Vector> {
        let uv = v.to_unit();
        let dt = uv.dot(n);
        let discriminant = 1. - ni_over_nt * ni_over_nt * (1. - dt * dt);

        if discriminant > 0. {
            return Some((uv - n.scale(dt)).scale(ni_over_nt) - n.scale(discriminant.sqrt()));
        }
        None
    }
}

impl Reflect for Dielectric {
    fn reflect(&self, v: Vector, n: Vector) -> Vector {
        v - n.scale(2. * v.dot(n))
    }
}

fn schlick(cosine: Float, refraction_index: Float) -> Float {
    let r0 = (1. - refraction_index) / (1. + refraction_index);
    let r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cosine).powf(5.)
}

impl Scatter for Dielectric {
    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<Bounce> {
        let outward_normal: Vector;
        let ni_over_nt: Float;
        let cosine: Float;
        let reflected = self.reflect(ray.direction, intersection.normal);

        if ray.direction.dot(intersection.normal) > 0. {
            outward_normal = -intersection.normal;
            ni_over_nt = self.refraction_index;
            cosine = self.refraction_index * ray.direction.dot(intersection.normal)
                / ray.direction.len();
        } else {
            outward_normal = intersection.normal;
            ni_over_nt = 1. / self.refraction_index;
            cosine = -ray.direction.dot(intersection.normal) / ray.direction.len();
        }

        if let Some(refracted) = self.refract(ray.direction, outward_normal, ni_over_nt) {
            Some(Bounce {
                attenuation: WHITE,
                scattered: {
                    if 0.5 < schlick(cosine, self.refraction_index) {
                        Ray {
                            origin: intersection.p,
                            direction: reflected,
                        }
                    } else {
                        Ray {
                            origin: intersection.p,
                            direction: refracted,
                        }
                    }
                },
            })
        } else {
            Some(Bounce {
                attenuation: WHITE,
                scattered: Ray {
                    origin: intersection.p,
                    direction: reflected,
                },
            })
        }
    }
}
