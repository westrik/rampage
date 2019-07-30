use crate::geometry::ray::Ray;
use crate::geometry::vector::*;
use crate::util::random::*;
use crate::{float, Float};

#[derive(Debug, PartialEq)]
pub struct Camera {
    pub origin: Vector,
    pub lower_left: Vector,
    pub horizontal: Vector,
    pub vertical: Vector,
    pub u: Vector,
    pub v: Vector,
    pub w: Vector,
    pub lens_radius: Float,
}

pub trait RayGenerator {
    fn get_ray(&self, s: Float, t: Float) -> Ray;
}

impl RayGenerator for Camera {
    fn get_ray(&self, s: Float, t: Float) -> Ray {
        let rd = random_vector_in_unit_disk().scale(self.lens_radius);
        let offset = self.u.scale(rd.x) + self.v.scale(rd.y);

        Ray {
            origin: self.origin + offset,
            direction: self.lower_left + self.horizontal.scale(s) + self.vertical.scale(t)
                - self.origin
                - offset,
        }
    }
}

// TODO: add documentation.
pub fn build_camera(
    origin: Vector,
    to: Vector,
    vup: Vector,
    vertical_fov: Float,
    aspect: Float,
    aperture: Float,
    focus_distance: Float,
) -> Camera {
    let lens_radius = aperture / 2.;
    let theta = vertical_fov * float::consts::PI / 180.;
    let half_height = (theta / 2.).tan();
    let half_width = aspect * half_height;

    let w = (origin - to).to_unit();
    let u = vup.cross(w).to_unit();
    let v = w.cross(u);

    Camera {
        origin,
        lower_left: origin
            - u.scale(half_width * focus_distance)
            - v.scale(half_height * focus_distance)
            - w.scale(focus_distance),
        horizontal: u.scale(2. * half_width * focus_distance),
        vertical: v.scale(2. * half_height * focus_distance),
        u,
        v,
        w,
        lens_radius,
    }
}

#[cfg(test)]
mod test_camera {
    use super::*;
    use crate::geometry::vector::test_vectors::*;

    #[test]
    fn test_camera_setup() {
        let camera = build_camera(
            Vector {
                x: 13.,
                y: 2.,
                z: 3.,
            },
            Vector {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            Vector {
                x: 0.,
                y: 1.,
                z: 0.,
            },
            20.,
            200. / 150.,
            0.1,
            10.,
        );

        assert_eq!(
            camera.origin,
            Vector {
                x: 13.,
                y: 2.,
                z: 3.
            }
        );
        assert_eq!(
            camera.lower_left,
            Vector {
                x: 3.0898184957721693,
                y: -1.2262841980681716,
                z: 3.12585077292591
            }
        );
        assert_eq!(
            camera.horizontal,
            Vector {
                x: 1.0573012773276516,
                y: -0.0,
                z: -4.581638868419823
            }
        );
        assert_eq!(
            camera.vertical,
            Vector {
                x: -0.5094205020606202,
                y: 3.4875711294919385,
                z: -0.11755857739860466
            }
        );
        assert_eq!(
            camera.u,
            Vector {
                x: 0.22485950669875845,
                y: -0.0,
                z: -0.97439119569462
            }
        );
        assert_eq!(
            camera.v,
            Vector {
                x: -0.14445336159384606,
                y: 0.9889499370655616,
                z: -0.0333353911370414
            }
        );
        assert_eq!(
            camera.w,
            Vector {
                x: 0.9636241116594315,
                y: 0.14824986333222023,
                z: 0.22237479499833035
            }
        );
        assert_eq!(camera.lens_radius, 0.05);
    }
}
