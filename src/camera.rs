use crate::{
    color::{self, Color},
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::*,
    pdf::{HittablePdf, MixturePdf, Pdf},
    ray::Ray,
    rtweekend::{self, *},
    vec3::{Point3, Vec3},
};

use crate::Rc;
use indicatif::ProgressBar;
use std::io::{self, BufWriter, Write};

/// Camera struct containing parameters for rendering a 3D scene.
pub struct Camera {
    /// Ratio of image width over height
    pub aspect_ratio: f64,
    /// Rendered image width in pixels
    pub img_width: u32,
    /// Number of stochastic samples per pixel
    pub samples_per_pixel: u32,
    /// Maximum number of ray bounces allowed
    pub max_depth: u32,
    /// Background color of the scene
    pub background: Color,
    /// Whether to use a gradient sky as background
    pub use_gradient_sky: bool,
    /// Vertical field of view (in degrees)
    pub vfov: f64,
    /// Camera position
    pub lookfrom: Point3,
    /// Target point the camera looks at
    pub lookat: Point3,
    /// "Up" direction vector relative to camera
    pub vup: Vec3,

    /// Variation angle for depth of field (defocus blur)
    pub defocus_angle: f64,
    /// Distance to plane of perfect focus
    pub focus_dist: f64,

    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,

    image_height: u32,
    pixel_samples_scale: f64,
    sqrt_spp: i32,
    recip_sqrt_spp: f64,
    center: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3, // Camera coordinate frame basis vectors
}

impl Camera {
    /// Initialize a new camera with given parameters and compute derived values.
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
            sqrt_spp: 0,
            recip_sqrt_spp: 0.0,
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

// Public methods
impl Camera {
    /// Render the scene described by `world` and `lights` hittables.
    pub fn render(&mut self, world: &dyn Hittable, lights: Rc<dyn Hittable>) {
        let stdout = io::stdout();
        let mut out = BufWriter::new(stdout.lock());

        writeln!(out, "P3").unwrap();
        writeln!(out, "{} {}", self.img_width, self.image_height).unwrap();
        writeln!(out, "255").unwrap();

        let bar = ProgressBar::new(self.image_height as u64);

        for j in 0..self.image_height {
            for i in 0..self.img_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                // Stratified sampling over sqrt(spp) x sqrt(spp) subpixels
                for s_j in 0..self.sqrt_spp {
                    for s_i in 0..self.sqrt_spp {
                        let r = self.get_ray(i, j, s_i, s_j);
                        pixel_color += self.ray_color(&r, self.max_depth, world, lights.clone());
                    }
                }
                let pc = self.pixel_samples_scale * pixel_color;
                color::write_color(&mut out, &pc).unwrap();
            }
            bar.inc(1);
        }
        bar.finish();
        out.flush().unwrap();
    }
}

// Private methods
impl Camera {
    /// Compute derived camera parameters based on initial settings.
    fn initialize(&mut self) {
        self.image_height = ((self.img_width as f64) / self.aspect_ratio) as u32;
        if self.image_height < 1 {
            self.image_height = 1;
        }

        self.sqrt_spp = (self.samples_per_pixel as f64).sqrt() as i32;
        self.pixel_samples_scale = 1.0 / ((self.sqrt_spp * self.sqrt_spp) as f64);
        self.recip_sqrt_spp = 1.0 / (self.sqrt_spp as f64);

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

    /// Compute color for a ray by tracing through the scene recursively.
    fn ray_color(
        &self,
        r: &Ray,
        depth: u32,
        world: &dyn Hittable,
        lights: Rc<dyn Hittable>,
    ) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::default();

        if !world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
            return self.background;
        }

        let mut srec = ScatterRecord::new();

        let color_from_emission = if let Some(mat) = &rec.mat {
            mat.emitted(r, &rec, rec.u, rec.v, &rec.p)
        } else {
            Color::new(0.0, 0.0, 0.0)
        };

        if let Some(mat) = &rec.mat {
            if !mat.scatter(r, &rec, &mut srec) {
                return color_from_emission;
            }
        } else {
            return color_from_emission;
        }

        if srec.skip_pdf {
            return srec.attenuation
                * self.ray_color(&srec.skip_pdf_ray, depth - 1, world, lights.clone());
        }

        let light_ptr = Rc::new(HittablePdf::new(lights.clone(), &rec.p));
        let p = MixturePdf::new(light_ptr, srec.pdf_ptr.as_ref().unwrap().clone());

        let scattered = Ray::new(rec.p, p.generate(), r.time());
        let pdf_value = p.value(&scattered.direction());

        let scattering_pdf = if let Some(mat) = &rec.mat {
            mat.scattering_pdf(r, &rec, &scattered)
        } else {
            0.0
        };

        let sample_color = self.ray_color(&scattered, depth - 1, world, lights.clone());
        let color_from_scatter = (srec.attenuation * scattering_pdf * sample_color) / pdf_value;

        color_from_emission + color_from_scatter
    }

    /// Returns the background color based on the sky gradient setting.
    fn get_background_color(&self, r: &Ray) -> Color {
        if self.use_gradient_sky {
            let unit_direction = Vec3::unit_vector(r.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);

            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        } else {
            self.background
        }
    }

    /// Returns a camera ray through pixel (i, j) with subpixel offsets (s_i, s_j).
    fn get_ray(&self, i: u32, j: u32, s_i: i32, s_j: i32) -> Ray {
        let offset = Camera::sample_square_stratified(self, s_i, s_j);

        let pixel_sample = self.pixel00_loc
            + (((i as f64) + offset.x()) * self.pixel_delta_u)
            + (((j as f64) + offset.y()) * self.pixel_delta_v);
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            Camera::defocus_disk_sample(self)
        };
        let ray_direction = pixel_sample - ray_origin;

        let ray_time = random_double();

        Ray::new(ray_origin, ray_direction, ray_time)
    }

    /// Samples a random point within the defocus disk for depth-of-field effects.
    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }

    /// Returns a random point in the unit square centered at (0,0).
    fn sample_square() -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    /// Returns a stratified random point in the sub-pixel grid cell specified by s_i, s_j.
    fn sample_square_stratified(&self, s_i: i32, s_j: i32) -> Vec3 {
        let px = (((s_i as f64) + random_double()) * self.recip_sqrt_spp) - 0.5;
        let py = (((s_j as f64) + random_double()) * self.recip_sqrt_spp) - 0.5;

        Vec3::new(px, py, 0.0)
    }
}
