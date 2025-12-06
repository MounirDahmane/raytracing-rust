use crate::vec3::Vec3;
use crate::interval::Interval;
use std::io::Write;

pub type Color = Vec3;

#[inline]
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if(linear_component > 0.0){
        return  linear_component.sqrt();
    }
    return 0.0;
}
pub fn write_color<W: Write>(out: &mut W, pixel_color: &Color) -> std::io::Result<()> {

    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();
    

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    // Translate the [0,1] component values to the byte range [0,255].

    let intensity = Interval::new(0.000, 0.999);

    let rbyte: u8 = (255.999 * intensity.clamp(r)) as u8;
    let gbyte: u8 = (255.999 * intensity.clamp(g)) as u8;
    let bbyte: u8 = (255.999 * intensity.clamp(b)) as u8;

    // Write out the pixel color components.
    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte)?;

    Ok(())
}
