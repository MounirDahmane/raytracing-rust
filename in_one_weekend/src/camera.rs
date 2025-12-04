use crate::{
    color,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    rtweekend::INFINITY,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub img_width: u32,
}

impl Camera {
    pub fn init(aspect_ratio: f64, img_width: u32) -> Self {
        Camera {
            aspect_ratio,
            img_width,
        }
    }
}

// public
impl Camera {
    pub fn render(&mut self, world: &dyn Hittable) {
        let (image_height, center, pixel_delta_u, pixel_delta_v, pixel00_loc) =
            Camera::initialize(self);

        println!("P3");
        println!("{} {}", self.img_width, image_height);
        println!("255");

        for i in 0..image_height {
            eprint!("\rScanlines remaining: {}", image_height - i - 1);
            for j in 0..self.img_width {
                let pixel_center =
                    pixel00_loc + ((j as f64) * pixel_delta_u) + ((i as f64) * pixel_delta_v);
                let ray_direction = pixel_center - center;

                let r = Ray::new(center, ray_direction);

                let pixel_color = Camera::ray_color(&r, world);
                color::write_color(&pixel_color);
            }
        }
        eprint!("                           \n");
        eprintln!("\rdone");
    }
}
// private
impl Camera {
    fn initialize(&mut self) -> (u32, Vec3, Vec3, Vec3, Vec3) {
        // Image

        // Calculate the image height, and ensure that it's at least 1.
        let mut image_height: u32 = ((self.img_width as f64) / self.aspect_ratio) as u32;
        if image_height < 1 {
            image_height = 1;
        } else {
            image_height = image_height;
        }

        // Camera
        let center = Point3::default(); // eye point

        let focal_length = 1.0; //[ CAMERA ] ---- distance ---- [ VIRTUAL SCREEN ]

        // Viewport width less than one are ok since they are real valued.
        let viewport_height = 2.0;
        let viewport_width = viewport_height * ((self.img_width / image_height) as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0); // Left edge to Right edge
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0); // Up edge to Down edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / (self.img_width) as f64;
        let pixel_delta_v = viewport_v / (image_height) as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc: Vec3 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        (
            image_height,
            center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
        )
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
}
