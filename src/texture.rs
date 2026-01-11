use std::sync::Arc;

use crate::{color::Color, interval::Interval, perlin::Perlin, rtw_image::RtwImage, vec3::Point3};

/// Trait for textures that provide a color at given (u, v) coordinates and 3D point.
pub trait Texture: Send + Sync {
    /// Returns the texture color at (u, v) and point p.
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

/// Texture representing a solid color.
pub struct SolidColor {
    /// The color value.
    pub albedo: Color,
}

impl SolidColor {
    #[inline(always)]
    pub fn new(albedo: &Color) -> Self {
        Self { albedo: *albedo }
    }

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

/// Checkerboard texture alternating between two sub-textures.
pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    /// Creates a checker texture from scale and two textures.
    #[inline(always)]
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    /// Creates a checker texture from scale and two solid colors.
    #[inline(always)]
    pub fn new_(scale: f64, c1: &Color, c2: &Color) -> Self {
        Self::new(
            scale,
            Arc::new(SolidColor::new(c1)),
            Arc::new(SolidColor::new(c2)),
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

/// Texture that maps an image onto geometry.
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
            // Cyan for missing texture as a debug color
            return Color::new(0.0, 1.0, 1.0);
        }

        // Clamp UV coords and flip v for image origin
        let u = Interval::new(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v);

        let i = (u * self.image.width() as f64) as i32;
        let j = (v * self.image.height() as f64) as i32;

        let pixel = self.image.pixel_data(i, j);
        let scale = 1.0 / 255.0;

        scale * Color::new(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64)
    }
}

/// Procedural noise texture using Perlin noise.
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
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
