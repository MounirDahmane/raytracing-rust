use crate::interval::Interval;
use crate::vec3::Vec3;
use std::io::Write;

pub type Color = Vec3;

/// Converts a linear color component to gamma-corrected value (sqrt for gamma 2.0)
#[inline]
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

/// Writes a color to output in ASCII PPM format after gamma correction and clamping.
pub fn write_color<W: Write>(out: &mut W, pixel_color: &Color) -> std::io::Result<()> {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Apply gamma correction
    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    // Clamp color components to [0, 0.999] before scaling to [0, 255]
    let intensity = Interval::new(0.0, 0.999);

    let rbyte: u8 = (255.999 * intensity.clamp(r)) as u8;
    let gbyte: u8 = (255.999 * intensity.clamp(g)) as u8;
    let bbyte: u8 = (255.999 * intensity.clamp(b)) as u8;

    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte)?;

    Ok(())
}
