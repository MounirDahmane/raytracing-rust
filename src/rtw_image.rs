use image::io::Reader as ImageReader;
use image::DynamicImage;
use std::path::{Path, PathBuf};

fn srgb_to_linear_channel(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

fn float_to_byte(value: f32) -> u8 {
    if value <= 0.0 {
        0
    } else if value >= 1.0 {
        255
    } else {
        (256.0 * value) as u8
    }
}

fn clamp_i32(x: i32, low: i32, high: i32) -> i32 {
    if x < low {
        low
    } else if x < high {
        x
    } else {
        high - 1
    }
}

pub struct RtwImage {
    fdata: Option<Vec<f32>>, // linear floats r,g,b,...
    bdata: Option<Vec<u8>>,  // bytes built from fdata
    w: usize,
    h: usize,
    bytes_per_scanline: usize,
}

impl RtwImage {
    pub fn new() -> Self {
        Self {
            fdata: None,
            bdata: None,
            w: 0,
            h: 0,
            bytes_per_scanline: 0,
        }
    }

    pub fn with_filename(filename: &str) -> Self {
        let mut img = Self::new();
        let p1 = PathBuf::from(filename);
        let p2 = PathBuf::from("images").join(filename);

        if img.load(&p1).is_err() {
            let _ = img.load(&p2); // ignore error; method will leave img empty if fails
        }
        img
    }

    /// Load a single path, build fdata (linear floats) and bdata (bytes).
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

        // build bdata from fdata using the same conversion rule
        let f = self.fdata.as_ref().unwrap();
        let mut bvec = Vec::with_capacity(w * h * 3);
        for &v in f.iter() {
            bvec.push(float_to_byte(v));
        }
        self.bdata = Some(bvec);

        Ok(())
    }

    pub fn width(&self) -> usize {
        if self.fdata.is_some() {
            self.w
        } else {
            0
        }
    }
    pub fn height(&self) -> usize {
        if self.fdata.is_some() {
            self.h
        } else {
            0
        }
    }

    /// Returns [u8;3] RGB for clamped pixel (magenta if no image)
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

    /// Returns linear [f32;3] RGB for clamped pixel (magenta in linear if no image)
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
