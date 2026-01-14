use crate::{
    rtweekend::random_int_range,
    vec3::{Point3, Vec3},
};

const POINT_COUNT: usize = 256;

pub struct Perlin {
    randvec: [Vec3; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    /// Creates a new Perlin noise generator with random unit vectors and permutation tables.
    pub fn new() -> Self {
        let mut randvec = [Vec3::default(); POINT_COUNT];
        for item in randvec.iter_mut().take(POINT_COUNT) {
            *item = Vec3::unit_vector(Vec3::random_range(-1.0, 1.0));
        }

        let perm_x = Perlin::perlin_generate_perm();
        let perm_y = Perlin::perlin_generate_perm();
        let perm_z = Perlin::perlin_generate_perm();

        Self {
            randvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    /// Compute Perlin noise value at point p.
    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as usize;
        let j = p.y().floor() as usize;
        let k = p.z().floor() as usize;

        // Gather the eight surrounding gradient vectors from the permutation tables
        let mut c = [[[Vec3::default(); 2]; 2]; 2];
        for (di, c_di) in c.iter_mut().enumerate() {
            for (dj, c_dj) in c_di.iter_mut().enumerate() {
                for (dk, c_dk) in c_dj.iter_mut().enumerate() {
                    let idx = self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255];
                    *c_dk = self.randvec[idx];
                }
            }
        }

        Perlin::perlin_interp(c, u, v, w)
    }

    /// Turbulence: sum of absolute values of noise over multiple frequencies.
    pub fn turb(&self, p: &Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }
}

impl Perlin {
    /// Generates a permutation array of POINT_COUNT elements.
    fn perlin_generate_perm() -> [usize; POINT_COUNT] {
        let mut p = [0usize; POINT_COUNT];
        for (i, item) in p.iter_mut().enumerate().take(POINT_COUNT) {
            *item = i;
        }
        Perlin::permute(&mut p);
        p
    }

    /// Randomly shuffles the permutation array using Fisher-Yates algorithm.
    fn permute(p: &mut [usize; POINT_COUNT]) {
        for i in (1..POINT_COUNT).rev() {
            let j = random_int_range(0, i as i32) as usize;
            p.swap(i, j);
        }
    }

    /// Trilinear interpolation of scalar values.
    #[allow(dead_code)]
    fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_u = if i == 1 { u } else { 1.0 - u };
                    let weight_v = if j == 1 { v } else { 1.0 - v };
                    let weight_w = if k == 1 { w } else { 1.0 - w };

                    accum += weight_u * weight_v * weight_w * c[i][j][k];
                }
            }
        }
        accum
    }

    /// Perlin interpolation with Hermitian smoothing and gradient dot products.
    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        // Hermitian smoothing function
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    let weight_u = if i == 1 { uu } else { 1.0 - uu };
                    let weight_vv = if j == 1 { vv } else { 1.0 - vv };
                    let weight_ww = if k == 1 { ww } else { 1.0 - ww };

                    accum += weight_u * weight_vv * weight_ww * Vec3::dot(&c[i][j][k], &weight_v);
                }
            }
        }

        accum
    }
}
