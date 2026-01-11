use crate::{
    rtweekend::random_int_range,
    vec3::{Point3, Vec3},
};

const POINT_COUNT: usize = 256;

/// Perlin noise generator for procedural textures and turbulence effects.
pub struct Perlin {
    randvec: [Vec3; POINT_COUNT], // Random unit vectors for gradient noise
    perm_x: [usize; POINT_COUNT], // Permutation arrays for hashing coordinates
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    /// Creates a new Perlin noise instance with random gradients and permutations.
    pub fn new() -> Self {
        let mut randvec = [Vec3::default(); POINT_COUNT];
        for i in 0..POINT_COUNT {
            randvec[i] = Vec3::unit_vector(Vec3::random_range(-1.0, 1.0));
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

    /// Computes Perlin noise value at point `p`.
    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - (p.x()).floor();
        let v = p.y() - (p.y()).floor();
        let w = p.z() - (p.z()).floor();

        let i = (p.x().floor()) as i32;
        let j = (p.y().floor()) as i32;
        let k = (p.z().floor()) as i32;

        // Gather gradient vectors at cube corners using permutation hashing
        let mut c = [[[Vec3::default(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.randvec[self.perm_x[((i as usize) + di) & 255]
                        ^ self.perm_y[((j as usize) + dj) & 255]
                        ^ self.perm_z[((k as usize) + dk) & 255]];
                }
            }
        }

        Perlin::perlin_interp(c, u, v, w)
    }

    /// Computes turbulence by summing noise over multiple octaves.
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
    /// Generates a random permutation array of 0..255.
    fn perlin_generate_perm() -> [usize; POINT_COUNT] {
        let mut p = [0usize; POINT_COUNT];
        for i in 0..POINT_COUNT {
            p[i] = i;
        }
        Perlin::permute(&mut p);
        p
    }

    /// Shuffles array in place using Fisher-Yates algorithm.
    fn permute(p: &mut [usize; POINT_COUNT]) {
        for i in (1..POINT_COUNT).rev() {
            let j = i as i32;
            let target = random_int_range(0, j);
            p.swap(i, target as usize);
        }
    }

    /// Trilinear interpolation of scalar values at the corners of a cube.
    fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += ((i as f64) * u + ((1 - i) as f64) * (1.0 - u))
                        * ((j as f64) * v + ((1 - j) as f64) * (1.0 - v))
                        * ((k as f64) * w + ((1 - k) as f64) * (1.0 - w))
                        * c[i][j][k];
                }
            }
        }
        accum
    }

    /// Interpolates noise contribution from gradient vectors using smoothing.
    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - (i as f64), v - (j as f64), w - (k as f64));

                    accum += ((i as f64) * uu + ((1 - i) as f64) * (1.0 - uu))
                        * ((j as f64) * vv + ((1 - j) as f64) * (1.0 - vv))
                        * ((k as f64) * ww + ((1 - k) as f64) * (1.0 - ww))
                        * Vec3::dot(&c[i][j][k], &weight_v);
                }
            }
        }

        accum
    }
}
