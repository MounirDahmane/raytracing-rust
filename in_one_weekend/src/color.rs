use crate::vec3::Vec3;
use crate::interval::Interval;
use std::io::Write;

pub type Color = Vec3;

pub fn write_color<W: Write>(out: &mut W, pixel_color: &Color) -> std::io::Result<()> {

    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // Translate the [0,1] component values to the byte range [0,255].

    let intensity = Interval::new(0.000, 0.999);

    let rbyte: u8 = (255.999 * intensity.clamp(r)) as u8;
    let gbyte: u8 = (255.999 * intensity.clamp(g)) as u8;
    let bbyte: u8 = (255.999 * intensity.clamp(b)) as u8;

    // Write out the pixel color components.
    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte)?;

    Ok(())
}
