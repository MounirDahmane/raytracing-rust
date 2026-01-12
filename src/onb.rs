use crate::vec3::Vec3;

pub struct Onb {
    axis: (Vec3, Vec3, Vec3),
}

impl Onb {
    pub fn new(n: &Vec3) -> Self {
        let w = Vec3::unit_vector(*n);
        let a = if w.x().abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v = Vec3::unit_vector(w.cross(&a));
        let u = Vec3::cross(&w, &v);

        Self { axis: (u, v, w) }
    }

    pub fn u(&self) -> Vec3 {
        self.axis.0
    }
    pub fn v(&self) -> Vec3 {
        self.axis.1
    }
    pub fn w(&self) -> Vec3 {
        self.axis.2
    }

    pub fn transform(&self, v: &Vec3) -> Vec3 {
        let (u, v_axis, w) = self.axis;
        v[0] * u + v[1] * v_axis + v[2] * w
    }
}
