// adding multithreading, maybe with rayon
// add supporting command-line arguments for the multiple scenes later
// Scene Modularization
// Your scene functions are nicely modularized. You could store them in a list of functions or use a dispatch map to select them dynamically.
// readme mn chatGbt
// try to load everything then do the job to eliminate the memory bounds

mod camera;
mod color;
mod interval;
mod ray;
mod rtweekend;
mod vec3;

mod aabb;
mod bvh;
mod rtw_image;
mod constant_medium;
mod perlin;
mod quad;

use crate::constant_medium::ConstantMedium;
use crate::quad::Primitive;
mod texture;

mod hittable;
mod hittable_list;
mod material;
mod sphere;

use crate::bvh::BvhNode;
use crate::color::Color;
use crate::hittable::{Hittable, RotateY, Translate};
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Material, Metal, DiffuseLight, Lambertian};
use crate::quad::Quad;
use crate::ray::Ray;
use crate::rtweekend::random_double_range;
use crate::sphere::Sphere;
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use crate::vec3::{Point3, Vec3};

use std::sync::Arc;

fn bouncing_spheres() {
    // World
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::new_(
        0.32,
        &Color::new(0.2, 0.3, 0.1),
        &Color::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new_static_sphere(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_(checker)),
    )));

    //let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    //world.add(Arc::new(Sphere::new_static_sphere(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rtweekend::random_double();
            let center = Point3::new(
                (a as f64) + 0.9 * rtweekend::random_double(),
                0.2,
                (b as f64) + 0.9 * rtweekend::random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;
                if choose_mat < 0.8 {
                    //diffuse
                    let albedo = Color::random() * Color::random();
                    sphere_material = Arc::new(Lambertian::new(albedo));
                    let center2 = center + Vec3::new(0.0, random_double_range(0.0, 0.5), 0.0);
                    world.add(Arc::new(Sphere::new_dynamic_sphere(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    //metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rtweekend::random_double_range(0.0, 0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    let center2 = center + Vec3::new(0.0, random_double_range(0.0, 0.5), 0.0);
                    world.add(Arc::new(Sphere::new_dynamic_sphere(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                } else {
                    //glass
                    sphere_material = Arc::new(Dielectric::new(1.5));

                    let center2 = center + Vec3::new(0.0, random_double_range(0.0, 0.5), 0.0);
                    world.add(Arc::new(Sphere::new_dynamic_sphere(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new_static_sphere(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new_static_sphere(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new_static_sphere(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let bvh_root = Arc::new(BvhNode::new_from_list(&mut world));
    let mut new_world = HittableList::new();
    new_world.add(bvh_root);

    let mut cam = camera::Camera::init(
        16.0 / 9.0,
        400,
        100,
        50,
        Color::new(0.70, 0.80, 1.00),
        true,
        20.0,
        
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.6,
    );
    cam.render(&new_world);
}

fn checkered_spheres() {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::new_(
        0.32,
        &Color::new(0.2, 0.3, 0.1),
        &Color::new(0.9, 0.9, 0.9),
    ));

    world.add(Arc::new(Sphere::new_static_sphere(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new_(checker.clone())),
    )));
    world.add(Arc::new(Sphere::new_static_sphere(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new_(checker.clone())),
    )));

    let mut cam = camera::Camera::init(
        16.0 / 9.0,
        400,
        100,
        50,
        Color::new(0.70, 0.80, 1.00),
        false,
        20.0,
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
    );

    cam.render(&world);
}

fn earth() {
    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::new_(earth_texture));
    let globe = Arc::new(Sphere::new_static_sphere(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        earth_surface,
    ));

    let mut cam = camera::Camera::init(
        16.0 / 9.0,
        400,
        100,
        50,
        Color::new(0.70, 0.80, 1.00),
        false,
        20.0,
        Point3::new(0.0, 0.0, 12.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
    );

    let mut world = HittableList::new();
    world.add(globe);

    cam.render(&world);
}

fn perlin_spheres() {
    let mut world = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));

    world.add(Arc::new(Sphere::new_static_sphere(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_(pertext.clone())),
    )));

    world.add(Arc::new(Sphere::new_static_sphere(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new_(pertext.clone())),
    )));

    let mut cam = camera::Camera::init(
        16.0 / 9.0,
        400,
        100,
        50,
        Color::new(0.70, 0.80, 1.00),
        false,
        20.0,
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
    );

    cam.render(&world);
}

fn quads() {
    let mut world = HittableList::new();

    // Materials
    let left_red = Arc::new(Lambertian::new(Color::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::new(Color::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::new(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::new(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::new(Color::new(0.2, 0.8, 0.8)));

    // Quads (use Primitive::Quad to preserve original behaviour)
    world.add(Arc::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
        Primitive::Quad,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
        Primitive::Quad,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
        Primitive::Quad,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
        Primitive::Quad,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
        Primitive::Quad,
    )));

    let mut cam = camera::Camera::init(
        16.0 / 9.0,
        400,
        100,
        50,
        Color::new(0.70, 0.80, 1.00),
        false,
        80.0,
        Point3::new(0.0, 0.0, 9.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
    );

    cam.render(&world);
}

fn test_all_primitives() {
    let mut world = HittableList::new();

    // distinct materials for clarity
    let red = Arc::new(Lambertian::new(Color::new(1.0, 0.2, 0.2)));
    let green = Arc::new(Lambertian::new(Color::new(0.2, 1.0, 0.2)));
    let blue = Arc::new(Lambertian::new(Color::new(0.2, 0.2, 1.0)));
    let orange = Arc::new(Lambertian::new(Color::new(1.0, 0.6, 0.2)));
    let purple = Arc::new(Lambertian::new(Color::new(0.6, 0.2, 1.0)));
    let gray = Arc::new(Lambertian::new(Color::new(0.6, 0.6, 0.6)));
    let yellow = Arc::new(Lambertian::new(Color::new(1.0, 0.9, 0.2)));

    // Base local edge vectors: each quad spans 2x2 in local (a,b) space so radius 1 disk fits nicely
    let u = Vec3::new(2.0, 0.0, 0.0);
    let v = Vec3::new(0.0, 2.0, 0.0);

    // Row layout
    let z = 5.0;
    let y = -1.0;

    // 1) Quad (control)
    world.add(Arc::new(Quad::new(
        Point3::new(-9.0, y, z),
        u,
        v,
        red.clone(),
        Primitive::Quad,
    )));

    // 2) Disk radius 1.0 (in a,b space), will yield a circle inside quad
    world.add(Arc::new(Quad::new(
        Point3::new(-6.0, y, z),
        u,
        v,
        green.clone(),
        Primitive::Disk(1.0),
    )));

    // 3) Triangle
    world.add(Arc::new(Quad::new(
        Point3::new(-3.0, y, z),
        u,
        v,
        blue.clone(),
        Primitive::Triangle,
    )));

    // 4) Ellipse rx=1.0 ry=0.6
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, y, z),
        u,
        v,
        orange.clone(),
        Primitive::Ellipse { rx: 1.0, ry: 0.6 },
    )));

    // 5) Annulus inner 0.5 outer 0.95 (ring)
    world.add(Arc::new(Quad::new(
        Point3::new(3.0, y, z),
        u,
        v,
        purple.clone(),
        Primitive::Annulus {
            inner: 0.5,
            outer: 0.95,
        },
    )));

    // 6) Texture mask: use procedural checker that implements your Texture trait
    let checker_tex = Arc::new(CheckerTexture::new_(
        8.0, // scale (your constructor name may vary)
        &Color::new(1.0, 1.0, 1.0),
        &Color::new(0.0, 0.0, 0.0),
    ));

    // pass it into a quad primitive
    let quad = Quad::new(
        Point3::new(6.0, y, z),
        u,
        v,
        gray.clone(),
        Primitive::TextureMask(checker_tex),
    );

    // 7) Mandelbrot region
    world.add(Arc::new(Quad::new(
        Point3::new(7.0, y, 2.0),
        u,
        v,
        yellow.clone(),
        Primitive::Mandelbrot { iterations: 256 },
    )));

    // Camera: back up so entire row fits
    let mut cam = camera::Camera::init(
        16.0 / 9.0,
        1200, // wider to fit the row
        100,
        50,
        Color::new(0.70, 0.80, 1.00),
        false,
        80.0, // vfov
        Point3::new(0.0, 0.0, 12.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
    );

    cam.render(&world);
}

fn simple_light() {
    let mut world = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(Sphere::new_static_sphere(Point3::new(0.0, -1000.0, 0.0),
     1000.0, Arc::new(Lambertian::new_(pertext.clone())))));
     
    world.add(Arc::new(Sphere::new_static_sphere(Point3::new(0.0, 2.0, 0.0),
     2.0, Arc::new(Lambertian::new_(pertext.clone())))));

    let difflight = Arc::new(DiffuseLight::new_(&Vec3::new(4.0, 4.0, 4.0)));
    world.add(Arc::new(Quad::new(Point3::new(3.0, 1.0, -2.0),
         Vec3::new(2.0, 0.0, 0.0), Vec3::new(0.0, 2.0, 0.0), difflight.clone(), Primitive::Quad)));

    world.add(Arc::new(Sphere::new_static_sphere(Point3::new(0.0, 7.0, 0.0),
     2.0, difflight.clone())));

    let mut cam = camera::Camera::init(
        16.0 / 9.0,
        400,
        100,
        50,
        Color::new(0.0, 0.0, 0.0),
        false,
        20.0,
        Point3::new(26.0, 3.0, 6.0),
        Point3::new(0.0, 2.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
    );
    cam.render(&world);
}

fn cornell_box_shape() {
    let mut world = HittableList::new();

    let red   = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_(&Vec3::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(Quad::new(Point3::new(555.0, 0.0,   0.0  ), Vec3::new(0.0,555.0,0.0 ), Vec3::new(0.0,0.0,555.0 ), green, Primitive::Quad)));
    world.add(Arc::new(Quad::new(Point3::new(0.0,   0.0,   0.0  ), Vec3::new(0.0,555.0,0.0 ), Vec3::new(0.0,0.0,555.0 ), red,   Primitive::Quad)));
    world.add(Arc::new(Quad::new(Point3::new(343.0, 554.0, 332.0), Vec3::new(-130.0,0.0,0.0), Vec3::new(0.0,0.0,-105.0), light, Primitive::Quad)));
    world.add(Arc::new(Quad::new(Point3::new(0.0,   0.0,   0.0  ), Vec3::new(555.0,0.0,0.0 ), Vec3::new(0.0,0.0,555.0 ), white.clone(), Primitive::Quad)));
    world.add(Arc::new(Quad::new(Point3::new(555.0, 555.0, 555.0), Vec3::new(-555.0,0.0,0.0), Vec3::new(0.0,0.0,-555.0), white.clone(), Primitive::Quad)));
    world.add(Arc::new(Quad::new(Point3::new(0.0,   0.0,   555.0), Vec3::new(555.0,0.0,0.0 ), Vec3::new(0.0,555.0,0.0 ), white.clone(), Primitive::Quad)));

    
    let mut box_shape1: Arc<dyn Hittable> = Arc::new(Quad::box_shape(
    &Point3::new(0.0, 0.0, 0.0),
    &Point3::new(165.0, 330.0, 165.0),
    white.clone(),
    ));
    
    box_shape1 = Arc::new(RotateY::new(box_shape1, 15.0));
    box_shape1 = Arc::new(Translate::new(box_shape1, Vec3::new(265.0, 0.0, 295.0)));

    world.add(box_shape1);


    let mut box_shape2: Arc<dyn Hittable> = Arc::new(Quad::box_shape(
    &Point3::new(0.0, 0.0, 0.0),
    &Point3::new(165.0, 165.0, 165.0),
    white.clone(),
    ));
    
    box_shape2 = Arc::new(RotateY::new(box_shape2, -18.0));
    box_shape2 = Arc::new(Translate::new(box_shape2, Vec3::new(130.0, 0.0, 65.0)));

    world.add(box_shape2);

    
    let mut cam = camera::Camera::init(
        1.0,
        600,
        200,
        50,
        Color::new(0.0, 0.0, 0.0),
        false,
        40.0,
        Point3::new(278.0, 278.0, -800.0),
        Point3::new(278.0, 278.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
    );

    cam.render(&world);
}

fn cornell_smoke() {
    let mut world = HittableList::new();

    let red   = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_(&Vec3::new(7.0, 7.0, 7.0)));

    // Right wall (green)
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green.clone(),
        Primitive::Quad,
    )));

    // Left wall (red)
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red.clone(),
        Primitive::Quad,
    )));

    // Light (ceiling source) — matches C++: point(113,554,127), vec3(330,0,0), vec3(0,0,305)
    world.add(Arc::new(Quad::new(
        Point3::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        light.clone(),
        Primitive::Quad,
    )));

    // Ceiling (white)
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 555.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
        Primitive::Quad,
    )));

    // Floor (white)
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
        Primitive::Quad,
    )));

    // Back wall (white) at z = 555
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
        Primitive::Quad,
    )));

    // box_shape 1
    let mut box_shape1: Arc<dyn Hittable> = Arc::new(Quad::box_shape(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    box_shape1 = Arc::new(RotateY::new(box_shape1, 15.0));
    box_shape1 = Arc::new(Translate::new(box_shape1, Vec3::new(265.0, 0.0, 295.0)));

    // box_shape 2
    let mut box_shape2: Arc<dyn Hittable> = Arc::new(Quad::box_shape(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    box_shape2 = Arc::new(RotateY::new(box_shape2, -18.0));
    box_shape2 = Arc::new(Translate::new(box_shape2, Vec3::new(130.0, 0.0, 65.0)));

    // Smoke volumes
    world.add(Arc::new(ConstantMedium::new_(box_shape1, 0.01, &Color::new(0.0, 0.0, 0.0))));
    world.add(Arc::new(ConstantMedium::new_(box_shape2, 0.01, &Color::new(1.0, 1.0, 1.0))));

    // Camera: aspect 1.0, width 600, samples 200, max_depth 50
    let mut cam = camera::Camera::init(
        1.0,                                 // aspect_ratio
        600,                                 // image_width (changed to 600)
        200,                                 // samples_per_pixel
        50,                                  // max_depth
        Color::new(0.0, 0.0, 0.0),           // background
        false,                               // (retained your boolean arg)
        40.0,                                // vfov
        Point3::new(278.0, 278.0, -800.0),   // lookfrom
        Point3::new(278.0, 278.0, 0.0),      // lookat
        Vec3::new(0.0, 1.0, 0.0),            // vup
        0.0,                                 // defocus / aperture
    );

    cam.render(&world);
}


fn final_scene(image_width: u32, samples_per_pixel: u32, max_depth: u32) {
    
    let mut box_shapees1 = HittableList::new();
    let albedo = Color::new(0.48, 0.83, 0.53);
    let ground = Arc::new(Lambertian::new(albedo));

    let box_shapees_per_side = 20;

    for i in 0..box_shapees_per_side {
        for j in 0..box_shapees_per_side {
            let w = 100.0;
            let x0 = -1000.0 + (i as f64) * w;
            let z0 = -1000.0 + (j as f64) * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rtweekend::random_double_range(1.0, 101.0);
            let z1 = z0 + w;
            
            box_shapees1.add(Arc::new(Quad::box_shape(&Point3::new(x0, y0, z0), &Point3::new(x1, y1, z1), ground.clone())));
        }
    }

    let mut world = HittableList::new();
    
    world.add(Arc::new(BvhNode::new_from_list(&mut box_shapees1)));

    let light = Arc::new(DiffuseLight::new_(&Color::new(7.0, 7.0, 7.0)));
    world.add(Arc::new(Quad::new(Point3::new(123.0,554.0,147.0), Vec3::new(300.0,0.0,0.0 ), 
            Vec3::new(0.0,0.0,265.0 ), light, Primitive::Quad)));
    
    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0,0.0,0.0);

    let albedo = Color::new(0.7, 0.3, 0.1);
    let sphere_material = Arc::new(Lambertian::new(albedo));

    world.add(Arc::new(Sphere::new_dynamic_sphere(center1, center2, 50.0, sphere_material)));

    let static_center = Point3::new(260.0, 150.0, 45.0);
    world.add(Arc::new(Sphere::new_static_sphere(static_center, 50.0, Arc::new(Dielectric::new(1.5)))));

    let static_center = Point3::new(0.0, 150.0, 145.0);
    world.add(Arc::new(Sphere::new_static_sphere(static_center, 50.0, 
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)))));

    let static_center = Point3::new(360.0, 150.0, 145.0);
    let boundary = Arc::new(Sphere::new_static_sphere(static_center, 70.0, Arc::new(Dielectric::new(1.5))));
    
    world.add(boundary.clone());
    
    let albedo = Color::new(0.2, 0.4, 0.9);
    world.add(Arc::new(ConstantMedium::new_(boundary, 0.2, &albedo)));
    
    let static_center = Point3::new(0.0, 0.0, 0.0);
    let boundary = Arc::new(Sphere::new_static_sphere(static_center, 5000.0, Arc::new(Dielectric::new(1.5))));
    let albedo = Color::new(1.0, 1.0, 1.0);

    world.add(Arc::new(ConstantMedium::new_(boundary, 0.0001, &albedo)));
    
    let tex = Arc::new(ImageTexture::new("earthmap.jpg"));
    let emat = Arc::new(Lambertian::new_(tex));

    let static_center = Point3::new(400.0, 200.0, 400.0);
    world.add(Arc::new(Sphere::new_static_sphere(static_center, 100.0, emat)));

    let pertext = Arc::new(NoiseTexture::new(0.2));
    let static_center = Point3::new(220.0, 280.0, 300.0);

    world.add(Arc::new(Sphere::new_static_sphere(static_center, 80.0, Arc::new(Lambertian::new_(pertext)))));

    let mut box_shapees2 = HittableList::new();
    
    let albedo = Color::new(0.73, 0.73, 0.73);
    let white = Arc::new(Lambertian::new(albedo));

    let ns = 1000;
    for _ in 0..ns {
        box_shapees2.add(Arc::new(Sphere::new_static_sphere(Point3::random_range(0.0, 165.0), 10.0, white.clone())));
    }

    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BvhNode::new_from_list(&mut box_shapees2)), 
            15.0)), 
        Vec3::new(-100.0, 270.0, 395.0)
    )));

    let mut cam = camera::Camera::init(
        1.0,
        image_width,
        samples_per_pixel,
        max_depth,
        Color::new(0.0, 0.0, 0.0),
        false,
        40.0,
        Point3::new(478.0, 278.0, -600.0),
        Point3::new(278.0, 278.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
    );

    cam.render(&world);
}


fn main() {
    let x = 10;

    match x {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => test_all_primitives(),
        7 => simple_light(),
        8 => cornell_box_shape(),
        9 => cornell_smoke(),
        10 => final_scene(800, 500, 40),
        _ => print!(""),
    };

}
