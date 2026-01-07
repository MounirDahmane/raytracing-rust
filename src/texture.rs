use std::rc::Rc;

use crate::{
    color::Color,
    interval::Interval,
    perlin::Perlin,
    rtw_image::RtwImage,
    vec3::Point3,
};

/// Trait for texture types that can provide a color value at given texture coordinates and position.
pub trait Texture {
    /// Returns the color value of the texture at coordinates `(u, v)` and point `p`.
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

/// A texture representing a single solid color.
pub struct SolidColor {
    /// The color value of the texture.
    pub albedo: Color,
}

impl SolidColor {
    /// Creates a new `SolidColor` from a reference to a `Color`.
    #[inline(always)]
    pub fn new(albedo: &Color) -> Self {
        Self { albedo: *albedo }
    }

    /// Creates a new `SolidColor` from red, green, and blue components.
    #[inline(always)]
    pub fn new_(red: f64, green: f64, blue: f64) -> Self {
        Self {
            albedo: Color::new(red, green, blue),
        }
    }
}

impl Texture for SolidColor {
    #[inline(always)]
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.albedo
    }
}

/// A checkerboard texture alternating between two sub-textures.
pub struct CheckerTexture {
    inv_scale: f64,
    even: Rc<dyn Texture>,
    odd: Rc<dyn Texture>,
}

impl CheckerTexture {
    /// Creates a new `CheckerTexture` from scale and two boxed textures.
    #[inline(always)]
    pub fn new(scale: f64, even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    /// Creates a new `CheckerTexture` from scale and two solid colors.
    #[inline(always)]
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
        let x_int = (self.inv_scale * p.x()).floor() as i32;
        let y_int = (self.inv_scale * p.y()).floor() as i32;
        let z_int = (self.inv_scale * p.z()).floor() as i32;

        let is_even = (x_int + y_int + z_int) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

/// A texture that maps an image file onto geometry.
pub struct ImageTexture {
    image: RtwImage,
}

impl ImageTexture {
    /// Loads an image texture from a file.
    pub fn new(filename: &str) -> Self {
        ImageTexture {
            image: RtwImage::with_filename(filename),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        if self.image.height() <= 0 {
            // Return cyan color for missing texture (debugging aid)
            return Color::new(0.0, 1.0, 1.0);
        }

        // Clamp texture coordinates to [0, 1], flip v for image coordinates
        let u = Interval::new(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v);

        let i = (u * self.image.width() as f64) as i32;
        let j = (v * self.image.height() as f64) as i32;

        let pixel = self.image.pixel_data(i, j);
        let scale = 1.0 / 255.0;

        scale * Color::new(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64)
    }
}

/// A procedural noise texture using Perlin noise.
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    /// Creates a new `NoiseTexture` with given scale (frequency).
    #[inline(always)]
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
