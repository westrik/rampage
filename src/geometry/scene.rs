use crate::color::Color;
use crate::geometry::shape::Shape;
use crate::geometry::sphere::Sphere;
use crate::geometry::vector::Vector;
use crate::material::lambertian::Lambertian;
use crate::material::Material;

pub fn random_scene() -> Vec<Shape> {
    let mut shapes = Vec::new();
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

    // Make large globe (lambertian)
    // Add random small spheres in loop (random texture)
    // Add three large balls (glass, lambertian, metal)
    /*
    std::vector<std::shared_ptr<Shape> > shape_list;

    std::shared_ptr<Shape> globe (new Sphere(Vector(0, -1000, 0), 1000,
        std::shared_ptr<Lambertian>(new Lambertian(Colour(0.5, 0.5, 0.5)))));
    shape_list.push_back(globe);

    for (auto a = -11; a < 11; a++) {
        for (auto b = -11; b < 11; b++) {
            double choose_material = drand48();
            Vector centre(a + 0.9 * drand48(), 0.2, b + 0.9 * drand48());
            if ((centre - Vector(4, 0.2, 0)).length() > 0.9) {
                if (choose_material < 0.8) {
                    // Diffuse
                    std::shared_ptr<Shape> diffuse_ball (new Sphere(centre, 0.2,
                        std::shared_ptr<Lambertian>(new Lambertian(Colour(drand48()*drand48(),
                        drand48()*drand48(), drand48()*drand48())))));
                    shape_list.push_back(diffuse_ball);
                } else if (choose_material < 0.95) {
                    // Metal
                    std::shared_ptr<Shape> metal_ball (new Sphere(centre, 0.2,
                        std::shared_ptr<Metal>(new Metal(
                            Colour(0.5*(1+drand48()), 0.5*(1+drand48()), 0.5*(1+drand48())),
                            0.5*drand48()))));
                    shape_list.push_back(metal_ball);
                } else {
                    // Glass
                    std::shared_ptr<Shape> glass_ball (new Sphere(centre, 0.2,
                        std::shared_ptr<Dielectric>(new Dielectric(1.5))));
                    shape_list.push_back(glass_ball);
                }
            }
        }
    }

    std::shared_ptr<Shape> big_glass_ball (new Sphere(Vector(0, 1, 0), 1,
        std::shared_ptr<Dielectric>(new Dielectric(1.5))));
    shape_list.push_back(big_glass_ball);

    std::shared_ptr<Shape> big_diffuse_ball (new Sphere(Vector(-4, 1, 0), 1,
        std::shared_ptr<Lambertian>(new Lambertian(Colour(0.4, 0.2, 0.1)))));
    shape_list.push_back(big_diffuse_ball);

    std::shared_ptr<Shape> big_metal_ball (new Sphere(Vector(4, 1, 0), 1,
        std::shared_ptr<Metal>(new Metal(Colour(0.7, 0.6, 0.5), 0))));
    shape_list.push_back(big_metal_ball);

    return std::make_shared<ShapeList> (shape_list);
    }
    */

    shapes
}
