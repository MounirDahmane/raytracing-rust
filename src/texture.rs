use std::rc::Rc;

use crate::{color::Color, interval::Interval, perlin::Perlin, rtw_image::RtwImage, vec3::Point3};

/// Trait for textures that provide a color value given texture coordinates and a point in space.
pub trait Texture {
    /// Returns the color value at texture coordinates `(u, v)` and point `p`.
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

/// A texture that is a solid color everywhere.
pub struct SolidColor {
    pub albedo: Color,
}

impl SolidColor {
    /// Creates a new solid color texture from a `Color`.
    pub fn new(albedo: &Color) -> Self {
        Self { albedo: *albedo }
    }

    /// Creates a new solid color texture from RGB components.
    pub fn new_(red: f64, green: f64, blue: f64) -> Self {
        Self {
            albedo: Color::new(red, green, blue),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.albedo
    }
}

/// A checkerboard texture that alternates between two textures based on position.
pub struct CheckerTexture {
    inv_scale: f64,
    even: Rc<dyn Texture>,
    odd: Rc<dyn Texture>,
}

impl CheckerTexture {
    /// Creates a checker texture with the given scale and two textures for the even and odd squares.
    pub fn new(scale: f64, even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    /// Creates a checker texture with solid colors for even and odd squares.
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

        let is_even = (x_integer + y_integer + z_integer) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

/// A texture that maps an image onto surfaces.
pub struct ImageTexture {
    image: RtwImage,
}

impl ImageTexture {
    /// Creates an image texture by loading the image from a file.
    pub fn new(filename: &str) -> Self {
        ImageTexture {
            image: RtwImage::with_filename(filename),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        // Return solid cyan if no image data is available (debugging aid).
        if self.image.height() <= 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        // Clamp texture coordinates to [0,1] and flip V for image coordinates.
        let u = Interval::new(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v);

        let i = (u * self.image.width() as f64) as i32;
        let j = (v * self.image.height() as f64) as i32;

        let pixel = self.image.pixel_data(i, j);

        let color_scale = 1.0 / 255.0;

        color_scale * Color::new(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64)
    }
}

/// A procedural texture based on Perlin noise.
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64, // Frequency scale
}

impl NoiseTexture {
    /// Creates a new noise texture with the given scale.
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        Color::new(0.5, 0.5, 0.5)
            * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(p, 7)).sin())
    }
}
