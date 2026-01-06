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

use rayon::prelude::*;
use std::sync::Arc;

pub struct Camera {
    pub aspect_ratio: f64,
    pub img_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,

    pub defocus_angle: f64,
    pub focus_dist: f64,

    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,

    image_height: u32,
    pixel_samples_scale: f64,
    center: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    /// Creates a new Camera and initializes derived values.
    pub fn init(
        aspect_ratio: f64,
        img_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        vfov: f64,
    ) -> Self {
        let mut camera = Camera {
            aspect_ratio,
            img_width,
            samples_per_pixel,
            max_depth,
            vfov,
            lookfrom: Point3::new(13.0, 2.0, 3.0),
            lookat: Point3::new(0.0, 0.0, 0.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.6,
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
}

// Public interface
impl Camera {
    /// Render the scene by tracing rays and writing colors to stdout in PPM format.
    pub fn render(&mut self, world: &(dyn Hittable + Sync)) {
        let stdout = io::stdout();
        let mut out = BufWriter::new(stdout.lock());

        writeln!(out, "P3").unwrap();
        writeln!(out, "{} {}", self.img_width, self.image_height).unwrap();
        writeln!(out, "255").unwrap();

        let bar = ProgressBar::new(self.image_height as u64);
        let bar = Arc::new(bar);

        // Parallel iterate over rows. We produce Vec<Vec<Color>>: one Vec<Color> per row.
        // Use a descending row order if you want top-to-bottom consistent with previous render.
        let rows: Vec<Vec<Color>> = (0..self.image_height as usize)
            .into_par_iter()
            .map_init(
                || Arc::clone(&bar),
                |bar_clone, j_usize| {
                    let j = j_usize as u32;
                    // Each thread will render one row `j`.
                    let mut row: Vec<Color> = Vec::with_capacity(self.img_width as usize);

                    for i in 0..self.img_width {
                        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                        for _ in 0..self.samples_per_pixel {
                            let r = self.get_ray(i, j);
                            pixel_color += Camera::ray_color(&r, self.max_depth, world);
                        }
                        pixel_color *= self.pixel_samples_scale;
                        row.push(pixel_color);
                    }

                    // update progress bar per-row (thread-safe)
                    bar_clone.inc(1);
                    row
                },
            )
            .collect();

        bar.finish();

        // Write rows sequentially in the same order we rendered them.
        for row in rows {
            for pixel_color in row {
                color::write_color(&mut out, &pixel_color).unwrap();
            }
        }

        out.flush().unwrap();
    }
}

// Private helpers
impl Camera {
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

    #[inline]
    fn ray_color(r: &Ray, depth: u32, world: &dyn Hittable) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::default();

        if world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();

            if let Some(mat) = &rec.mat {
                if mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                    return attenuation * Camera::ray_color(&scattered, depth - 1, world);
                } else {
                    return Color::new(0.0, 0.0, 0.0);
                }
            }
        }

        let unit_direction = Vec3::unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);

        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    #[inline]
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

        Ray::new(ray_origin, ray_direction)
    }

    #[inline]
    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }

    #[inline]
    fn sample_square() -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }
}
