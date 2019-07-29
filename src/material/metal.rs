use crate::color::Color;
use crate::geometry::ray::Ray;
use crate::geometry::shape::Intersection;
use crate::material::Scatter;
use crate::Float;

pub struct Metal {
    pub albedo: Color,
    pub fuzziness: Float,
}

impl Scatter for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        intersection: &Intersection,
        attenuation: Color,
        scattered: &Ray,
    ) -> bool {
        unimplemented!()
    }
}
