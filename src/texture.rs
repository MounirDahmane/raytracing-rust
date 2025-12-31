use std::rc::Rc;

use crate::{
    color::{self, Color},
    interval::Interval,
    rtw_image::RtwImage,
    vec3::Point3,
};

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

pub struct SolidColor {
    pub albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: &Color) -> Self {
        Self { albedo: *albedo }
    }
    pub fn new_(red: f64, green: f64, blue: f64) -> Self {
        Self {
            albedo: Color::new(red, green, blue),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        return self.albedo;
    }
}

pub struct CheckerTexture {
    inv_scale: f64,
    even: Rc<dyn Texture>,
    odd: Rc<dyn Texture>,
}
impl CheckerTexture {
    pub fn new(scale: f64, even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }
    pub fn new_(scale: f64, c1: &Color, c2: &Color) -> Self {
        Self::new(
            scale,
            Rc::new(SolidColor::new(c1)),
            Rc::new(SolidColor::new(c2)),
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let x_integer = (self.inv_scale * p.x()).floor() as i32;
        let y_integer = (self.inv_scale * p.y()).floor() as i32;
        let z_integer = (self.inv_scale * p.z()).floor() as i32;

        let is_even = if (x_integer + y_integer + z_integer) % 2 == 0 {
            true
        } else {
            false
        };

        if is_even == true {
            return self.even.value(u, v, p);
        } else {
            return self.odd.value(u, v, p);
        }
    }
}

pub struct ImageTexture {
    image: RtwImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        ImageTexture {
            image: RtwImage::with_filename(filename),
        }
    }
}
impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        // If we have no texture data, then return solid cyan as a debugging aid.
        if self.image.height() <= 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        // Clamp input texture coordinates to [0,1] x [1,0]
        let u = Interval::new(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v); // Flip V to image coordinates

        let i = (u * self.image.width() as f64) as i32;
        let j = (v * self.image.height() as f64) as i32;

        let pixel = self.image.pixel_data(i, j);

        let color_scale = 1.0 / 255.0;

        return color_scale * Color::new(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64);
    }
}
