mod camera;
mod color;
mod interval;
mod ray;
mod rtweekend;
mod vec3;

mod onb;
mod pdf;

mod aabb;
mod bvh;
mod rtw_image;
mod constant_medium;
mod perlin;
mod quad;
use crate::quad::Primitive;
mod texture;

mod hittable;
mod hittable_list;
mod material;
mod sphere;

use crate::color::Color;
use crate::hittable::{Hittable, RotateY, Translate};
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, DiffuseLight, Material, Metal, lambertian, noMaterial};
use crate::quad::Quad;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3};

use std::rc::Rc;


fn main() {
    let mut world = HittableList::new();

    let red   = Rc::new(lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Rc::new(lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Rc::new(lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::new_(&Vec3::new(15.0, 15.0, 15.0)));

    world.add(Rc::new(Quad::new(Point3::new(555.0, 0.0,   0.0  ), Vec3::new(0.0,555.0,0.0 ), Vec3::new(0.0,0.0,555.0 ), green, Primitive::Quad)));
    world.add(Rc::new(Quad::new(Point3::new(0.0,   0.0,   0.0  ), Vec3::new(0.0,555.0,0.0 ), Vec3::new(0.0,0.0,555.0 ), red,   Primitive::Quad)));
    world.add(Rc::new(Quad::new(Point3::new(343.0, 554.0, 332.0), Vec3::new(-130.0,0.0,0.0), Vec3::new(0.0,0.0,-105.0), light, Primitive::Quad)));
    world.add(Rc::new(Quad::new(Point3::new(0.0,   0.0,   0.0  ), Vec3::new(555.0,0.0,0.0 ), Vec3::new(0.0,0.0,555.0 ), white.clone(), Primitive::Quad)));
    world.add(Rc::new(Quad::new(Point3::new(555.0, 555.0, 555.0), Vec3::new(-555.0,0.0,0.0), Vec3::new(0.0,0.0,-555.0), white.clone(), Primitive::Quad)));
    world.add(Rc::new(Quad::new(Point3::new(0.0,   0.0,   555.0), Vec3::new(555.0,0.0,0.0 ), Vec3::new(0.0,555.0,0.0 ), white.clone(), Primitive::Quad)));

    let aluminum = Rc::new(Metal::new(Color::new(0.8, 0.85, 0.88), 0.0));
    
    let mut box1: Rc<dyn Hittable> = Rc::new(Quad::Box(
    &Point3::new(0.0, 0.0, 0.0),
    &Point3::new(165.0, 330.0, 165.0),
    white.clone(),
    ));
    
    box1 = Rc::new(RotateY::new(box1, 15.0));
    box1 = Rc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));

    world.add(box1);


    // Glass Sphere
    let glass = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new_static_sphere(Point3::new(190.0, 90.0, 190.0), 
                90.0, glass)));

    // Light Sources
    let empty_material = Rc::new(noMaterial);
    let mut lights = HittableList::new();
    lights.add(
        Rc::new(Quad::new(Point3::new(343.0, 554.0, 332.0), 
        Vec3::new(-130.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -105.0), empty_material.clone(), Primitive::Quad)));

    
    lights.add(
        Rc::new(Sphere::new_static_sphere(Point3::new(190.0, 90.0, 190.0), 90.0, empty_material.clone())));

    

    let mut cam = camera::Camera::init(
        1.0,
        600,
        1000,
        50,
        Color::new(0.0, 0.0, 0.0),
        false,
        40.0,
        Point3::new(278.0, 278.0, -800.0),
        Point3::new(278.0, 278.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
    );

    cam.render(&world, Rc::new(lights));
    
}
