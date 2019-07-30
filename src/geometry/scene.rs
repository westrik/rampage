use crate::color::Color;
use crate::geometry::shape::Shape;
use crate::geometry::sphere::Sphere;
use crate::geometry::vector::*;
use crate::material::dielectric::Dielectric;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::material::Material;
use crate::Float;
use rand::prelude::*;

fn squared_rand(mut rng: ThreadRng) -> Float {
    rng.gen::<Float>() * rng.gen::<Float>()
}

pub fn random_scene() -> Vec<Shape> {
    let mut shapes = Vec::new();
    let mut rng = rand::thread_rng();

    // Make large sphere for the ground.
    let globe = Shape::Sphere(Sphere {
        center: Vector {
            x: 0.,
            y: -1000.,
            z: 0.,
        },
        radius: 1000.,
        material: Material::Lambertian(Lambertian {
            albedo: Color {
                r: 0.5,
                g: 0.5,
                b: 0.5,
            },
        }),
    });

    shapes.push(globe);

    // Add random small spheres in loop (random texture)
    for a in -11..11 {
        for b in -11..11 {
            let a_f = Float::from(a);
            let b_f = Float::from(b);
            let choose_material: Float = rng.gen();
            let center = Vector {
                x: a_f + 0.9 * rng.gen::<Float>(),
                y: 0.2,
                z: b_f + 0.9 * rng.gen::<Float>(),
            };
            if (center
                - Vector {
                    x: 4.,
                    y: 0.2,
                    z: 0.,
                })
            .len()
                > 0.9
            {
                if choose_material < 0.8 {
                    // Diffuse material
                    let diffuse_ball = Shape::Sphere(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Lambertian(Lambertian {
                            albedo: Color {
                                r: squared_rand(rng),
                                g: squared_rand(rng),
                                b: squared_rand(rng),
                            },
                        }),
                    });
                    shapes.push(diffuse_ball);
                } else if choose_material < 0.95 {
                    // Metal
                    let metal_ball = Shape::Sphere(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Metal(Metal {
                            albedo: Color {
                                r: 0.5 * (1. + rng.gen::<Float>()),
                                g: 0.5 * (1. + rng.gen::<Float>()),
                                b: 0.5 * (1. + rng.gen::<Float>()),
                            },
                            fuzziness: 0.5 * rng.gen::<Float>(),
                        }),
                    });
                    shapes.push(metal_ball);
                } else {
                    // Dielectric
                    let glass_ball = Shape::Sphere(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Dielectric(Dielectric {
                            refraction_index: 1.5,
                        }),
                    });
                    shapes.push(glass_ball);
                }
            }
        }
    }

    // Add three large balls (glass, lambertian, metal)
    let big_glass_ball = Shape::Sphere(Sphere {
        center: Vector {
            x: 0.,
            y: 1.,
            z: 0.,
        },
        radius: 1.0,
        material: Material::Dielectric(Dielectric {
            refraction_index: 1.5,
        }),
    });
    shapes.push(big_glass_ball);

    let big_diffuse_ball = Shape::Sphere(Sphere {
        center: Vector {
            x: -4.,
            y: 1.,
            z: 0.,
        },
        radius: 1.0,
        material: Material::Lambertian(Lambertian {
            albedo: Color {
                r: 0.4,
                g: 0.2,
                b: 0.1,
            },
        }),
    });
    shapes.push(big_diffuse_ball);

    let big_metal_ball = Shape::Sphere(Sphere {
        center: Vector {
            x: 4.,
            y: 1.,
            z: 0.,
        },
        radius: 1.0,
        material: Material::Metal(Metal {
            albedo: Color {
                r: 0.7,
                g: 0.6,
                b: 0.5,
            },
            fuzziness: 0.0,
        }),
    });
    shapes.push(big_metal_ball);

    shapes
}
