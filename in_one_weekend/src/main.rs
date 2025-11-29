mod color;
mod ray;
/// use cargo run --release > image.ppm
mod vec3;

use crate::color::{write_color, Color};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMG_WIDTH: u32 = 512;

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    let oc = *center - r.origin();
    let a = Vec3::dot(&r.direction(), &r.direction());
    let b = -2.0 * Vec3::dot(&r.direction(), &oc);
    let c = Vec3::dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    discriminant >= 0.0
}

fn ray_color(r: &Ray) -> Color {

    if hit_sphere(&Point3::new(0.0,0.0,-1.0), 0.5, r){
        return Color::new(255.0/255.0, 100.0/255.0, 255.0/255.0);
    }

    let unit_direction = Vec3::unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Calculate the image height, and ensure that it's at least 1.
    let mut image_height: u32 = ((IMG_WIDTH as f64) / ASPECT_RATIO) as u32;
    if image_height < 1 {
        image_height = 1;
    } else {
        image_height = image_height;
    }

    // Camera
    let focal_length = 1.0; //[ CAMERA ] ---- distance ---- [ VIRTUAL SCREEN ]

    // Viewport width less than one are ok since they are real valued.
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ((IMG_WIDTH / image_height) as f64);

    let camera_center = Point3::default();

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0); // Left edge to Right edge
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0); // Up edge to Down edge

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / (IMG_WIDTH) as f64;
    let pixel_delta_v = viewport_v / (image_height) as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render

    println!("P3");
    println!("{} {}", IMG_WIDTH, image_height);
    println!("255");

    for i in 0..image_height {
        eprint!("\rScanlines remaining: {}", image_height - i - 1);
        for j in 0..IMG_WIDTH {
            let pixel_center =
                pixel00_loc + ((j as f64) * pixel_delta_u) + ((i as f64) * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;

            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r);
            write_color(&pixel_color);
        }
    }
    eprint!("                           \n");
    eprintln!("\rdone");
}
