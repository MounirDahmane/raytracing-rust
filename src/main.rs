mod camera;
mod color;
mod interval;
mod ray;
mod rtweekend;
mod vec3;

mod hittable;
mod hittable_list;
mod material;
mod sphere;

use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Point3;

use std::rc::Rc;

fn main() {
    let mut world = HittableList::new();

    // Large ground sphere
    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    // Random small spheres scattered, avoid clustering near (4,0.2,0)
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rtweekend::random_double();
            let center = Point3::new(
                (a as f64) + 0.9 * rtweekend::random_double(),
                0.2,
                (b as f64) + 0.9 * rtweekend::random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material> = if choose_mat < 0.8 {
                    // Diffuse material
                    Rc::new(Lambertian::new(Color::random() * Color::random()))
                } else if choose_mat < 0.95 {
                    // Metal material with fuzz
                    Rc::new(Metal::new(
                        Color::random_range(0.5, 1.0),
                        rtweekend::random_double_range(0.0, 0.5),
                    ))
                } else {
                    // Dielectric (glass)
                    Rc::new(Dielectric::new(1.5))
                };
                world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    // Three big spheres with fixed materials
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Dielectric::new(1.5)),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
    )));

    // Initialize camera and render
    let mut cam = camera::Camera::init(16.0 / 9.0, 1920,
        1000, 50, 20.0);
    cam.render(&world);
}


// real    198m26.032s
// user    198m9.218s
// sys     0m6.954s