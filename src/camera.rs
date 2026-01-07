use crate::{
    color::{self, Color},
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    rtweekend::{self, *},
    vec3::{Point3, Vec3},
};

use indicatif::ProgressBar;
use std::io::{self, BufWriter, Write};

pub struct Camera {
    pub aspect_ratio: f64,      // Ratio of image width over height
    pub img_width: u32,         // Rendered image width in pixel count
    pub samples_per_pixel: u32, // Count of random samples per pixel
    pub max_depth: u32,         // Maximum number of ray bounces into scene
    pub background: Color,      // Scene background color
    pub use_gradient_sky: bool,
    pub vfov: f64,              // Vertical field of view in degrees
    pub lookfrom: Point3,       // Camera position
    pub lookat: Point3,         // Look-at target point
    pub vup: Vec3,              // "Up" direction for camera orientation

    pub defocus_angle: f64,     // Angle controlling depth of field (defocus)
    pub focus_dist: f64,        // Distance to focal plane

    defocus_disk_u: Vec3,       // Defocus disk horizontal vector
    defocus_disk_v: Vec3,       // Defocus disk vertical vector

    image_height: u32,
    pixel_samples_scale: f64,
    center: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,                   // Camera coordinate basis vectors
}

impl Camera {
    pub fn init(
        aspect_ratio: f64,
        img_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        background: Color,
        use_gradient_sky: bool,
        vfov: f64,
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        defocus_angle: f64,
    ) -> Self {
        let mut camera = Camera {
            aspect_ratio,
            img_width,
            samples_per_pixel,
            max_depth,
            background,
            use_gradient_sky,
            vfov,
            lookfrom,
            lookat,
            vup,
            defocus_angle,
            focus_dist: 10.0,
            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default(),

            image_height: 0,
            pixel_samples_scale: 0.0,
            center: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
            pixel00_loc: Vec3::default(),
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
        };
        camera.initialize();
        camera
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        let stdout = io::stdout();
        let mut out = BufWriter::new(stdout.lock());

        writeln!(out, "P3").unwrap();
        writeln!(out, "{} {}", self.img_width, self.image_height).unwrap();
        writeln!(out, "255").unwrap();

        let bar = ProgressBar::new(self.image_height as u64);

        for j in 0..self.image_height {
            for i in 0..self.img_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, world);
                }
                pixel_color *= self.pixel_samples_scale;

                color::write_color(&mut out, &pixel_color).unwrap();
            }
            bar.inc(1);
        }
        bar.finish();
        out.flush().unwrap();
    }

    fn initialize(&mut self) {
        self.image_height = ((self.img_width as f64) / self.aspect_ratio) as u32;
        if self.image_height < 1 {
            self.image_height = 1;
        }

        self.pixel_samples_scale = 1.0 / (self.samples_per_pixel as f64);

        self.center = self.lookfrom;

        let theta = rtweekend::degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.img_width as f64) / (self.image_height as f64);

        self.w = Vec3::unit_vector(self.lookfrom - self.lookat);
        self.u = Vec3::unit_vector(Vec3::cross(&self.vup, &self.w));
        self.v = Vec3::cross(&self.w, &self.u);

        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        self.pixel_delta_u = viewport_u / (self.img_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;

        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius =
            self.focus_dist * rtweekend::degrees_to_radians(self.defocus_angle / 2.0).tan();

        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn ray_color(&self, r: &Ray, depth: u32, world: &dyn Hittable) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::default();

        if !world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
            return self.get_background_color(r);
        }

        let mut scattered = Ray::default();
        let mut attenuation = Color::default();

        let color_from_emission = if let Some(mat) = &rec.mat {
            mat.emitted(rec.u, rec.v, &rec.p)
        } else {
            Color::new(0.0, 0.0, 0.0)
        };

        if let Some(mat) = &rec.mat {
            if !mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return color_from_emission;
            }
        } else {
            return color_from_emission;
        }

        let color_from_scatter = attenuation * self.ray_color(&scattered, depth - 1, world);

        color_from_emission + color_from_scatter
    }

    fn get_background_color(&self, r: &Ray) -> Color {
        if self.use_gradient_sky {
            let unit_direction = Vec3::unit_vector(r.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);

            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        } else {
            self.background
        }
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = Camera::sample_square();

        let pixel_sample = self.pixel00_loc
            + (((i as f64) + offset.x()) * self.pixel_delta_u)
            + (((j as f64) + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        let ray_direction = pixel_sample - ray_origin;

        let ray_time = random_double();

        Ray::new(ray_origin, ray_direction, ray_time)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }
}
