use std::rc::Rc;

use crate::{color::{self, Color}, vec3::Point3};

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}


pub struct SolidColor{
    pub albedo: Color,
}

impl SolidColor{

    pub fn new(albedo: &Color) -> Self {
        Self { albedo: *albedo }
    }
    pub fn new_(red: f64, green: f64, blue: f64) -> Self {
        Self { albedo: Color::new(red, green, blue) }
    }
}

impl Texture for SolidColor{
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color { return self.albedo;}
}


pub struct  CheckerTexture{
    inv_scale: f64,
    even: Rc<dyn Texture>,
    odd: Rc<dyn Texture>,
}
impl CheckerTexture{
    pub fn new(scale: f64, even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> Self {
        Self { inv_scale: 1.0/scale, 
                even, 
                odd 
            }
    }
    pub fn new_(scale: f64, c1: &Color, c2: &Color) -> Self {
        Self::new(
            scale, 
            Rc::new(SolidColor::new(c1)), 
            Rc::new(SolidColor::new(c2))
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color { 

        let x_integer = (self.inv_scale * p.x()).floor() as i32;
        let y_integer = (self.inv_scale * p.y()).floor() as i32;
        let z_integer = (self.inv_scale * p.z()).floor() as i32;

        let is_even = if (x_integer + y_integer + z_integer) % 2 == 0 {true} else {false}; 

        if is_even == true {
            return self.even.value(u, v, p);
        } 
        else {
            return self.odd.value(u, v, p) ;
        }
    }
}
