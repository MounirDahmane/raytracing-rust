use crate::{
    camera, color::{self, Color, write_color}, hittable::{HitRecord, Hittable}, interval::Interval, material::*, ray::Ray, rtweekend::{self, *}, vec3::{Point3, Vec3}
};

use indicatif::ProgressBar;
use std::io::{self, BufWriter, Write};
pub struct Camera {
    pub aspect_ratio: f64,
    pub img_width: u32,
    pub samples_per_pixel: u32,   // Count of random samples for each pixel
    pub max_depth: u32,   // Maximum number of ray bounces into scene
    pub vfov: f64 ,  // Vertical view angle (field of view)
    pub lookfrom: Point3,  // Point camera is looking from
    pub lookat  : Point3,  // Point camera is looking at
    pub vup     : Vec3,  // Camera-relative "up" direction

    pub defocus_angle: f64,   // Variation angle of rays through each pixel
    pub focus_dist: f64,   // Distance from camera lookfrom point to plane of perfect focus

    defocus_disk_u: Vec3,       // Defocus disk horizontal radius
    defocus_disk_v: Vec3,       // Defocus disk vertical radius
    
    image_height : u32,
    pixel_samples_scale: f64,
    center: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Vec3,     
    u: Vec3, v: Vec3, w: Vec3,   // Camera frame basis vectors
}
impl Camera {
    pub fn init(aspect_ratio: f64, img_width: u32, samples_per_pixel: u32, max_depth: u32, vfov: f64) -> Self {
        let mut camera = Camera {
            aspect_ratio,
            img_width,
            samples_per_pixel,
            max_depth,
            vfov,
            lookfrom: Point3::new(13.0,2.0,3.0),
            lookat: Point3::new(0.0,0.0,0.0), 
            vup: Vec3::new(0.0,1.0,0.0),       
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
            u: Vec3::default(), v: Vec3::default(), w: Vec3::default(),
        };
        camera.initialize(); // Compute derived data here
        camera
    }
}

// public
impl Camera {
    pub fn render(&mut self, world: &dyn Hittable) {

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
                    // For each pixel, take multiple stochastic samples (SSAA) and average their colors
                    let r = self.get_ray(i, j);
                    pixel_color += Camera::ray_color(&r, self.max_depth, world);
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
        self.center = self.lookfrom; // eye point
        
                //let focal_length = (self.lookfrom - self.lookat).length(); //[ CAMERA ] ---- distance ---- [ VIRTUAL SCREEN ]
        
        let theta = rtweekend::degrees_to_radians(self.vfov);
        let h = (theta/2.0).tan();

        // Viewport width less than one are ok since they are real valued.
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.img_width as f64) / (self.image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        self.w = Vec3::unit_vector(self.lookfrom - self.lookat);
        self.u = Vec3::unit_vector(Vec3::cross(&self.vup, &self.w));
        self.v = Vec3::cross(&self.w, &self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.

        let viewport_u = viewport_width * self.u;   // Vector across viewport horizontal edge
        let viewport_v = viewport_height * -self.v; // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / (self.img_width as f64) ;
        self.pixel_delta_v = viewport_v / (self.image_height as f64) ;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = self.center - (self.focus_dist * self.w) - viewport_u/2.0 - viewport_v/2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = self.focus_dist * rtweekend::degrees_to_radians(self.defocus_angle/2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;

    }
    fn ray_color(r: &Ray, depth: u32, world: &dyn Hittable) -> color::Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::default();

        if world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {

            let mut scattered = Ray::default();
            let mut attenuation = Color::default();

            if let Some(mat) = &rec.mat {
                if mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                    // continue ray
                    return  attenuation * Camera::ray_color(&scattered, depth - 1, world);
                } else {
                    // material absorbed the ray
                    return Color::new(0.0, 0.0, 0.0);
                }
            }
        }

        let unit_direction = Vec3::unit_vector(r.direction());
        let a: f64 = 0.5 * (unit_direction.y() + 1.0);
        
        // blendedValue
        (1.0 - a) * color::Color::new(1.0, 1.0, 1.0) + a * color::Color::new(0.5, 0.7, 1.0)
    }
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.

        // Generate a random offset within the pixel area to perform supersampling (SSAA).
        // This stochastic sampling reduces aliasing by averaging multiple rays per pixel
        // with slightly jittered positions instead of just shooting through the pixel center.

        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
                          + (((i as f64) + offset.x()) * self.pixel_delta_u)
                          + (((j as f64) + offset.y()) * self.pixel_delta_v);
        let ray_origin = if self.defocus_angle <= 0.0 {self.center } else {Camera::defocus_disk_sample(self)};
        let ray_direction = pixel_sample - ray_origin;

        let ray_time = random_double();
        
        Ray::new(ray_origin, ray_direction, ray_time)
    }
    fn defocus_disk_sample(&self) -> Point3{
        // Returns a random point in the camera defocus disk.
        let p = Vec3::random_in_unit_disk();
        return self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v);
    }
    fn sample_square() -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }
}


