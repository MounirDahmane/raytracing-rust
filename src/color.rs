use crate::interval::Interval;
use crate::vec3::Vec3;
use std::io::Write;

/// Alias for a color vector represented as Vec3 (RGB).
pub type Color = Vec3;

/// Converts a linear color component to gamma-corrected space using gamma=2.0 (sqrt).
#[inline]
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

/// Writes a color to the given output stream in PPM format after gamma correction and clamping.
///
/// # Arguments
///
/// * `out` - Output stream implementing `Write`.
/// * `pixel_color` - Color to write.
pub fn write_color<W: Write>(out: &mut W, pixel_color: &Color) -> std::io::Result<()> {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Replace NaN components with zero.
    if r.is_nan() {
        r = 0.0;
    }
    if g.is_nan() {
        g = 0.0;
    }
    if b.is_nan() {
        b = 0.0;
    }

    // Apply gamma correction.
    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    // Clamp components to [0, 0.999] and convert to [0,255] byte range.
    let intensity = Interval::new(0.000, 0.999);

    let rbyte: u8 = (255.999 * intensity.clamp(r)) as u8;
    let gbyte: u8 = (255.999 * intensity.clamp(g)) as u8;
    let bbyte: u8 = (255.999 * intensity.clamp(b)) as u8;

    // Write the RGB components as ASCII.
    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte)?;

    Ok(())
}
