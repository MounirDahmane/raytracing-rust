use crate::{onb, vec3::Vec3};

pub struct Onb {
    axis: (Vec3, Vec3, Vec3),
}
impl Onb {
    pub fn new(n: &Vec3) -> Self{
        let axis2 = Vec3::unit_vector(*n);
        let a = if (axis2.x()).abs() > 0.9 { Vec3::new(0.0, 1.0, 0.0)}
                else {
                    Vec3::new(1.0, 0.0, 0.0)
                };
        let axis1 = Vec3::unit_vector(axis2.cross(&a));
        let axis0 = Vec3::cross(&axis2, &axis1);

        Self { axis: (axis0, axis1, axis2) }
    }
}

impl Onb {

    pub fn u(&self) -> Vec3 { return self.axis.0; }
    pub fn v(&self) -> Vec3 { return self.axis.1; }
    pub fn w(&self) -> Vec3 { return self.axis.2; }

    pub fn transform(&self, v: &Vec3) -> Vec3 {
        // Transform from basis coordinates to local space.
        return (v[0] * self.axis.0) + (v[1] * self.axis.1) + (v[2] * self.axis.2);
    }
}
