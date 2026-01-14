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
mod constant_medium;
mod perlin;
mod quad;
mod rtw_image;
use crate::quad::Primitive;
mod texture;

mod hittable;
mod hittable_list;
mod material;
mod sphere;

use crate::color::Color;
use crate::hittable::{Hittable, RotateY, Translate};
use crate::hittable_list::HittableList;
use crate::material::{ColoredDielectric, Dielectric, DiffuseLight, Lambertian, Material, Metal};
use crate::quad::Quad;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::texture::{CheckerTexture, NoiseTexture, SolidColor};
use crate::vec3::{Point3, Vec3};

use std::sync::Arc;

fn main() {
    // Basic Lambertian materials for walls
    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));

    // Emissive materials for area and point lights
    let area_light_tex = Arc::new(SolidColor::new(&Color::new(18.0, 12.0, 5.0)));
    let area_light = Arc::new(DiffuseLight::new(area_light_tex.clone()));

    let point_light_tex = Arc::new(SolidColor::new(&Color::new(6.0, 5.0, 3.5)));
    let point_light = Arc::new(DiffuseLight::new(point_light_tex.clone()));

    // Checker texture for the floor
    let floor_tex = Arc::new(CheckerTexture::new_(
        15.0,
        &Color::new(0.83, 0.83, 0.83),
        &Color::new(0.55, 0.55, 0.50),
    ));
    let floor_mat = Arc::new(Lambertian::new_(floor_tex.clone()));

    // Metallic materials and colored dielectric materials
    let aluminum = Arc::new(Metal::new(Color::new(0.82, 0.86, 0.88), 0.0));
    let chrome = Arc::new(Metal::new(Color::new(0.98, 0.98, 1.00), 0.0));
    let blue_violet_glass = Arc::new(ColoredDielectric::new(1.5, Color::new(0.25, 0.14, 0.97)));
    let diamond = Arc::new(Dielectric::new(2.4));

    // Perlin noise texture for added surface detail
    let perlin_tex = Arc::new(NoiseTexture::new(4.0));
    let noisy_diffuse = Arc::new(Lambertian::new_(perlin_tex));

    let mut world = HittableList::new();

    // Left and right walls with classic Cornell box colors
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green.clone(),
        Primitive::Quad,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red.clone(),
        Primitive::Quad,
    )));

    // Ceiling emissive panel as area light
    world.add(Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        area_light.clone(),
        Primitive::Quad,
    )));

    // Floor with checker texture
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        floor_mat.clone(),
        Primitive::Quad,
    )));

    // Back wall and remaining walls with white Lambertian material
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
        Primitive::Quad,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
        Primitive::Quad,
    )));

    // Rotated chrome box, translated in the scene
    let mut box1: Arc<dyn Hittable + Send + Sync> = Arc::new(Quad::create_box(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 330.0, 165.0),
        chrome.clone(),
    ));
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    // Blue violet glass sphere
    world.add(Arc::new(Sphere::new_static_sphere(
        Point3::new(190.0, 90.0, 190.0),
        90.0,
        blue_violet_glass.clone(),
    )));

    // Small aluminum sphere for contrast
    world.add(Arc::new(Sphere::new_static_sphere(
        Point3::new(120.0, 50.0, 40.0),
        50.0,
        aluminum.clone(),
    )));

    // Noisy Perlin textured sphere for surface variation
    world.add(Arc::new(Sphere::new_static_sphere(
        Point3::new(420.0, 50.0, 220.0),
        50.0,
        noisy_diffuse.clone(),
    )));

    // Diamond sphere with high refractive index
    world.add(Arc::new(Sphere::new_static_sphere(
        Point3::new(500.0, 50.0, 40.0),
        50.0,
        diamond.clone(),
    )));

    // Large emissive sphere as accent light source
    world.add(Arc::new(Sphere::new_static_sphere(
        Point3::new(278.0, 500.0, 278.0),
        20.0,
        point_light.clone(),
    )));

    // Cluster of small spheres with randomized materials for complexity
    for a in 0..6 {
        for b in 0..6 {
            let choose_mat = rtweekend::random_double();
            let center = Point3::new(
                80.0 + a as f64 * 60.0 + 10.0 * rtweekend::random_double(),
                10.0 + 40.0 * rtweekend::random_double(),
                100.0 + b as f64 * 60.0 + 10.0 * rtweekend::random_double(),
            );

            // Avoid overlap with main glass sphere area
            let dist = ((center.x() - 190.0).powi(2) + (center.z() - 190.0).powi(2)).sqrt();
            if dist < 90.0 {
                continue;
            }

            if choose_mat < 0.6 {
                let albedo = Color::new(
                    rtweekend::random_double() * rtweekend::random_double(),
                    rtweekend::random_double() * rtweekend::random_double(),
                    rtweekend::random_double() * rtweekend::random_double(),
                );
                world.add(Arc::new(Sphere::new_static_sphere(
                    center,
                    12.0,
                    Arc::new(Lambertian::new(albedo)),
                )));
            } else if choose_mat < 0.9 {
                let albedo = Color::new(
                    0.5 * (1.0 + rtweekend::random_double()),
                    0.5 * (1.0 + rtweekend::random_double()),
                    0.5 * (1.0 + rtweekend::random_double()),
                );
                let fuzz = rtweekend::random_double() * 0.4;
                world.add(Arc::new(Sphere::new_static_sphere(
                    center,
                    10.0,
                    Arc::new(Metal::new(albedo, fuzz)),
                )));
            } else {
                world.add(Arc::new(Sphere::new_static_sphere(
                    center,
                    10.0,
                    Arc::new(Dielectric::new(1.5)),
                )));
            }
        }
    }

    // Lights used by the renderer for sampling direct illumination
    let mut lights = HittableList::new();
    lights.add(Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        area_light.clone(),
        Primitive::Quad,
    )));
    lights.add(Arc::new(Sphere::new_static_sphere(
        Point3::new(278.0, 500.0, 278.0),
        20.0,
        point_light.clone(),
    )));

    // Camera setup and rendering call
    let cam = camera::Camera::init(
        1.0,
        1200,  // higher resolution for better detail
        10000, // samples per pixel
        100,   // max bounce depth for complex lighting
        Color::new(0.0, 0.0, 0.0),
        false, // no DOF for sharpness
        40.0,
        Point3::new(278.0, 278.0, -800.0),
        Point3::new(278.0, 278.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
    );

    cam.render(&world, Arc::new(lights));
}
