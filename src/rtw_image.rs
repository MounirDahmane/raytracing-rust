use image::DynamicImage;
use image::io::Reader as ImageReader;
use std::path::{Path, PathBuf};

/// Converts an sRGB channel value to linear space.
fn srgb_to_linear_channel(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// Converts a floating point value in [0,1] to a byte in [0,255].
fn float_to_byte(value: f32) -> u8 {
    if value <= 0.0 {
        0
    } else if value >= 1.0 {
        255
    } else {
        (256.0 * value) as u8
    }
}

/// Clamps an i32 value to the range [low, high).
fn clamp_i32(x: i32, low: i32, high: i32) -> i32 {
    if x < low {
        low
    } else if x < high {
        x
    } else {
        high - 1
    }
}

/// Struct representing an image with linear float data and byte data.
pub struct RtwImage {
    fdata: Option<Vec<f32>>, // linear floats in RGB order
    bdata: Option<Vec<u8>>,  // bytes built from fdata
    w: usize,
    h: usize,
    bytes_per_scanline: usize,
}

impl RtwImage {
    /// Creates a new, empty image.
    pub fn new() -> Self {
        Self {
            fdata: None,
            bdata: None,
            w: 0,
            h: 0,
            bytes_per_scanline: 0,
        }
    }

    /// Creates an image and attempts to load it from a filename or from the "images" folder.
    pub fn with_filename(filename: &str) -> Self {
        let mut img = Self::new();
        let p1 = PathBuf::from(filename);
        let p2 = PathBuf::from("images").join(filename);

        if img.load(&p1).is_err() {
            let _ = img.load(&p2); // ignore error; image stays empty if loading fails
        }
        img
    }

    /// Loads an image from the given path, converting to linear floats and building byte data.
    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let dynimg: DynamicImage = ImageReader::open(&path)?.decode()?;
        let rgb8 = dynimg.to_rgb8();
        let (w, h) = (rgb8.width() as usize, rgb8.height() as usize);
        if w == 0 || h == 0 {
            return Err("empty image".into());
        }

        self.w = w;
        self.h = h;
        self.bytes_per_scanline = w * 3;

        let mut fvec = Vec::with_capacity(w * h * 3);
        for px in rgb8.pixels() {
            let r = (px[0] as f32) / 255.0;
            let g = (px[1] as f32) / 255.0;
            let b = (px[2] as f32) / 255.0;
            fvec.push(srgb_to_linear_channel(r));
            fvec.push(srgb_to_linear_channel(g));
            fvec.push(srgb_to_linear_channel(b));
        }
        self.fdata = Some(fvec);

        // Build byte data from linear floats
        let f = self.fdata.as_ref().unwrap();
        let mut bvec = Vec::with_capacity(w * h * 3);
        for &v in f.iter() {
            bvec.push(float_to_byte(v));
        }
        self.bdata = Some(bvec);

        Ok(())
    }

    /// Returns the image width in pixels, or 0 if no image is loaded.
    pub fn width(&self) -> usize {
        if self.fdata.is_some() { self.w } else { 0 }
    }

    /// Returns the image height in pixels, or 0 if no image is loaded.
    pub fn height(&self) -> usize {
        if self.fdata.is_some() { self.h } else { 0 }
    }

    /// Returns the RGB byte values of the pixel at `(x, y)`, clamping coordinates.
    ///
    /// Returns magenta `[255, 0, 255]` if no image data is available.
    pub fn pixel_data(&self, x: i32, y: i32) -> [u8; 3] {
        const MAGENTA: [u8; 3] = [255, 0, 255];
        if self.bdata.is_none() || self.w == 0 || self.h == 0 {
            return MAGENTA;
        }
        let xx = clamp_i32(x, 0, self.w as i32) as usize;
        let yy = clamp_i32(y, 0, self.h as i32) as usize;
        let idx = yy * self.bytes_per_scanline + xx * 3;
        let b = self.bdata.as_ref().unwrap();
        [b[idx], b[idx + 1], b[idx + 2]]
    }

    /// Returns the linear RGB float values of the pixel at `(x, y)`, clamping coordinates.
    ///
    /// Returns magenta `[1.0, 0.0, 1.0]` if no image data is available.
    pub fn pixel_linear(&self, x: i32, y: i32) -> [f32; 3] {
        const MAGENTA_F: [f32; 3] = [1.0, 0.0, 1.0];
        if self.fdata.is_none() || self.w == 0 || self.h == 0 {
            return MAGENTA_F;
        }
        let xx = clamp_i32(x, 0, self.w as i32) as usize;
        let yy = clamp_i32(y, 0, self.h as i32) as usize;
        let idx = yy * self.bytes_per_scanline + xx * 3;
        let f = self.fdata.as_ref().unwrap();
        [f[idx], f[idx + 1], f[idx + 2]]
    }
}
