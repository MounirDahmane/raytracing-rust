// further improvements: adding multithreading, maybe with rayon
// add supporting command-line arguments for the multiple scenes later

mod color;
mod ray;
mod rtweekend;
mod vec3;
mod camera;
mod interval;

mod aabb;
mod bvh;

mod teture;

mod hittable;
mod hittable_list;
mod sphere;
mod material;

use crate::bvh::BvhNode;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Material, Metal, lambertian, noMaterial};
use crate::ray::Ray;
use crate::rtweekend::{random_double, random_double_range};
use crate::sphere::Sphere;
use crate::teture::CheckerTexture;
use crate::vec3::{Point3, Vec3};
use crate::hittable::Hittable;

use std::rc::Rc;
use std::f64::consts::PI;
use std::sync::PoisonError;


fn bouncing_spheres(){
    // World
    let mut world = HittableList::new();

    let checker = Rc::new(CheckerTexture::new_(0.32, &Color::new(0.2, 0.3, 0.1), &Color::new(0.9, 0.9, 0.9)));
    world.add(Rc::new(Sphere::new_static_sphere(Point3::new(0.0, -1000.0, 0.0), 1000.0, Rc::new(lambertian::new_(checker)))));
    
    //let ground_material = Rc::new(lambertian::new(Color::new(0.5, 0.5, 0.5)));
    //world.add(Rc::new(Sphere::new_static_sphere(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));
    
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rtweekend::random_double();
            let center = Point3::new((a as f64) + 0.9 * rtweekend::random_double(), 0.2, (b as f64) + 0.9 * rtweekend::random_double());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material>;
                if choose_mat < 0.8{
                    //diffuse
                    let albedo = Color::random() * Color::random();
                    sphere_material = Rc::new(lambertian::new(albedo));
                    let center2 = center + Vec3::new(0.0, random_double_range(0.0, 0.5), 0.0);
                    world.add(Rc::new(Sphere::new_dynamic_sphere(center, center2, 0.2, sphere_material)));
                }
                else if choose_mat < 0.95 {
                    //metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rtweekend::random_double_range(0.0, 0.5);
                    sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    let center2 = center + Vec3::new(0.0, random_double_range(0.0, 0.5), 0.0);
                    world.add(Rc::new(Sphere::new_dynamic_sphere(center, center2, 0.2, sphere_material)));
                }
                else {
                    //glass
                    sphere_material = Rc::new(Dielectric::new(1.5));
                    
                    let center2 = center + Vec3::new(0.0, random_double_range(0.0, 0.5), 0.0);
                    world.add(Rc::new(Sphere::new_dynamic_sphere(center, center2, 0.2, sphere_material)));
                }
            }
        }
    }
    
    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new_static_sphere(Point3::new(0.0, 1.0, 0.0), 1.0, material1)));
    
    let material2 = Rc::new(lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new_static_sphere(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));
    
    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new_static_sphere(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));

    let bvh_root = Rc::new(BvhNode::new_from_list(&mut world));
    let mut new_world = HittableList::new();
    new_world.add(bvh_root);


    let mut cam = camera::Camera::init(16.0 / 9.0, 400, 100, 
                 50, 20.0, Point3::new(13.0,2.0,3.0), Point3::new(0.0,0.0,0.0),
                 Vec3::new(0.0,1.0,0.0), 0.6);
    cam.render(&new_world);

}

fn checkered_spheres() {
    
    let mut world = HittableList::new();

    let checker = Rc::new(CheckerTexture::new_(0.32, &Color::new(0.2, 0.3, 0.1), 
                &Color::new(0.9, 0.9, 0.9)));

    world.add(Rc::new(Sphere::new_static_sphere(Point3::new(0.0, -10.0, 0.0), 
                10.0, Rc::new(lambertian::new_(checker.clone())))));

    world.add(Rc::new(Sphere::new_static_sphere(Point3::new(0.0, 10.0, 0.0), 
                10.0, Rc::new(lambertian::new_(checker.clone())))));

    let mut cam = camera::Camera::init(16.0 / 9.0, 400, 100, 
                50, 20.0, Point3::new(13.0,2.0,3.0), Point3::new(0.0,0.0,0.0),
                Vec3::new(0.0,1.0,0.0), 0.0);

    cam.render(&world);
}
fn main() {

    let x = 2;
    match x {
    1 => bouncing_spheres(),
    2 => checkered_spheres(),
    _ => return,
};

}
