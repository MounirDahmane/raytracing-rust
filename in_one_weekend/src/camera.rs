use crate::{
    color::{self, Color, write_color},
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    rtweekend::*,
    vec3::{Point3, Vec3},
};

use indicatif::ProgressBar;
use std::io::{self, BufWriter, Write};

pub struct Camera {
    pub aspect_ratio: f64,
    pub img_width: u32,
    pub samples_per_pixel: u32,   // Count of random samples for each pixel

    image_height : u32,
    pixel_samples_scale: f64,
    center: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Vec3,
}

impl Camera {
    pub fn init(aspect_ratio: f64, img_width: u32, samples_per_pixel: u32) -> Self {
        let mut camera = Camera {
            aspect_ratio,
            img_width,
            samples_per_pixel,

            image_height: 0,
            pixel_samples_scale: 0.0,
            center: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
            pixel00_loc: Vec3::default(),
        };
        camera.initialize(); // Compute derived data here
        camera
    }
}

// public
impl Camera {
    pub fn render(&self, world: &dyn Hittable) {

        let stdout = io::stdout();
        let mut out = BufWriter::new(stdout.lock());

        writeln!(out, "P3").unwrap();
        writeln!(out, "{} {}", self.img_width, self.image_height).unwrap();
        writeln!(out, "255").unwrap();

        let bar = ProgressBar::new(self.image_height as u64);

        for j in 0..self.image_height {
            for i in 0..self.img_width {

                let mut pixel_color = color::Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Camera::ray_color(&r, world);
                }
                pixel_color *= self.pixel_samples_scale;
                
                color::write_color(&mut out, &pixel_color).unwrap();
            }
            bar.inc(1);
        }
        bar.finish();
        out.flush().unwrap();
    }
}
// private
impl Camera {

    fn initialize(&mut self){
        // Image

        // Calculate the image height, and ensure that it's at least 1.
        self.image_height = ((self.img_width as f64) / self.aspect_ratio) as u32;

        if self.image_height < 1 {
            self.image_height = 1;
        } else {
            self.image_height = self.image_height;
        }

        self.pixel_samples_scale = 1.0 / (self.samples_per_pixel as f64);  // Color scale factor for a sum of pixel samples

        // Camera
        self.center = Point3::default(); // eye point

        let focal_length = 1.0; //[ CAMERA ] ---- distance ---- [ VIRTUAL SCREEN ]

        // Viewport width less than one are ok since they are real valued.
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.img_width as f64) / (self.image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0); // Left edge to Right edge
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0); // Up edge to Down edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / (self.img_width as f64) ;
        self.pixel_delta_v = viewport_v / (self.image_height as f64) ;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

    }

    fn ray_color(r: &Ray, world: &dyn Hittable) -> color::Color {
        let mut rec = HitRecord::default();

        if world.hit(r, Interval::new(0.0, INFINITY), &mut rec) {
            return 0.5 * (rec.normal + color::Color::new(1.0, 1.0, 1.0));
        }

        let unit_direction = Vec3::unit_vector(r.direction());
        let a: f64 = 0.5 * (unit_direction.y() + 1.0);

        (1.0 - a) * color::Color::new(1.0, 1.0, 1.0) + a * color::Color::new(0.5, 0.7, 1.0)
        // blendedValue
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j.

        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
                          + (((i as f64) + offset.x()) * self.pixel_delta_u)
                          + (((j as f64) + offset.y()) * self.pixel_delta_v);
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)

    }
    fn sample_square() -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }
}
