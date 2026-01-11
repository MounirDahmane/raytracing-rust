use crate::{
    color::{self, Color},
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    rtweekend::{self, *},
    vec3::{Point3, Vec3},
};
use std::fs::File;
use std::path::Path;

use indicatif::ProgressBar;
use rayon::prelude::*;
use std::io::{self, BufWriter, Write};

pub struct Camera {
    pub aspect_ratio: f64,
    pub img_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub background: Color,
    pub use_gradient_sky: bool,
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
    /// Creates a new camera with given parameters and precomputes camera basis vectors.
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

    /// Renders the scene to stdout in PPM format with a progress bar.
    pub fn render(&mut self, world: &(dyn Hittable + Sync)) {
        let stdout = io::stdout();
        let mut out = BufWriter::new(stdout.lock());

        // Write PPM header
        let header = format!("P3\n{} {}\n255\n", self.img_width, self.image_height);
        out.write_all(header.as_bytes()).unwrap();

        let bar = ProgressBar::new(self.image_height as u64);

        let width = self.img_width;
        let height = self.image_height;
        let samples_per_pixel = self.samples_per_pixel;
        let max_depth = self.max_depth;
        let pixel_scale = self.pixel_samples_scale;

        let camera_ref = &*self;

        // Compute scanlines in parallel
        let mut rows: Vec<(u32, Vec<u8>)> = (0..height)
            .into_par_iter()
            .map(|j| {
                let mut buf = Vec::with_capacity((width as usize) * 16);
                for i in 0..width {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for _s in 0..samples_per_pixel {
                        let r = camera_ref.get_ray(i, j);
                        pixel_color += camera_ref.ray_color(&r, max_depth, world);
                    }
                    pixel_color *= pixel_scale;
                    color::write_color(&mut buf, &pixel_color).unwrap();
                }
                bar.inc(1);
                (j, buf)
            })
            .collect();

        // Sort rows to output in correct order
        rows.sort_by_key(|(j, _)| *j);

        // Write rows to stdout
        for (_j, row) in rows {
            out.write_all(&row).unwrap();
        }

        bar.finish();
        out.flush().unwrap();
    }

    /// Renders the scene to a PPM file.
    pub fn render_to_file(
        &mut self,
        world: &(dyn Hittable + Sync),
        idx: i32,
        filename: &str,
    ) -> io::Result<()> {
        let filename = format!("{}_{}.ppm", filename, idx);
        if let Some(parent) = Path::new(&filename).parent() {
            std::fs::create_dir_all(parent)?;
        }

        let file = File::create(&filename)?;
        let mut out = BufWriter::new(file);

        let header = format!("P3\n{} {}\n255\n", self.img_width, self.image_height);
        out.write_all(header.as_bytes())?;

        let bar = ProgressBar::new(self.image_height as u64);

        let width = self.img_width;
        let height = self.image_height;
        let samples_per_pixel = self.samples_per_pixel;
        let max_depth = self.max_depth;
        let pixel_scale = self.pixel_samples_scale;
        let camera_ref = &*self;

        // Parallel row computation
        let mut rows: Vec<(u32, Vec<u8>)> = (0..height)
            .into_par_iter()
            .map(|j| {
                let mut buf = Vec::with_capacity((width as usize) * 16);
                for i in 0..width {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for _ in 0..samples_per_pixel {
                        let r = camera_ref.get_ray(i, j);
                        pixel_color += camera_ref.ray_color(&r, max_depth, world);
                    }
                    pixel_color *= pixel_scale;
                    color::write_color(&mut buf, &pixel_color).unwrap();
                }
                bar.inc(1);
                (j, buf)
            })
            .collect();

        rows.sort_by_key(|(j, _)| *j);

        for (_j, row) in rows {
            out.write_all(&row)?;
        }

        bar.finish();
        out.flush()?;
        Ok(())
    }

    /// Computes camera parameters and coordinate basis vectors.
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

    /// Recursive ray color calculation with material scattering and emission.
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

    /// Returns background color, either a gradient sky or solid color.
    fn get_background_color(&self, r: &Ray) -> Color {
        if self.use_gradient_sky {
            let unit_direction = Vec3::unit_vector(r.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);

            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        } else {
            self.background
        }
    }

    /// Generates a ray from the camera through pixel (i, j), with optional defocus for depth of field.
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

    /// Samples a random point on the defocus disk for depth of field effect.
    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }

    /// Samples a random offset within a pixel square for anti-aliasing.
    fn sample_square() -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }
}
