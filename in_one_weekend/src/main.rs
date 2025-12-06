mod color;
mod ray;
mod rtweekend;
mod vec3;
mod camera;
mod interval;

mod hittable;
mod hittable_list;
mod sphere;

use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3};



fn main() {
    

    // World
    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let mut cam = camera::Camera::init(16.0 / 9.0, 400, 100, 50);
    
    cam.render(&world);

}
