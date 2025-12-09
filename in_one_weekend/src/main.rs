mod color;
mod ray;
mod rtweekend;
mod vec3;
mod camera;
mod interval;

mod hittable;
mod hittable_list;
mod sphere;
mod material;

use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Metal, lambertian};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3};

use std::rc::Rc;
use std::f64::consts::PI;

fn main() {

    // World
    let mut world = HittableList::new();

    let material_ground = Rc::new(lambertian::new(color::Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(lambertian::new(color::Color::new(0.1, 0.2, 0.5)));
    let material_left   = Rc::new(Dielectric::new(1.5));
    let material_bubble = Rc::new(Dielectric::new(1.0/1.5)); 
    let material_right       = Rc::new(Metal::new(color::Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(Sphere::new(Point3::new( 0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::new(Point3::new( 0.0,    0.0, -1.2),   0.5, material_center)));
    world.add(Box::new(Sphere::new(Point3::new(-1.0,    0.0, -1.0),   0.5, material_left)));
    world.add(Box::new(Sphere::new(Point3::new(-1.0,    0.0, -1.0),   0.4, material_bubble)));
    world.add(Box::new(Sphere::new(Point3::new( 1.0,    0.0, -1.0),   0.5, material_right)));

    let mut cam = camera::Camera::init(16.0 / 9.0, 400, 100, 50, 20.0);
    
    cam.render(&world);

}
