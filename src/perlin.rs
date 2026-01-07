use crate::{
    rtweekend::random_int_range,
    vec3::{Point3, Vec3},
};

const POINT_COUNT: usize = 256;

pub struct Perlin {
    randvec: [Vec3; POINT_COUNT],  // random unit vectors for noise
    perm_x: [usize; POINT_COUNT],  // permutation arrays for hashing
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut randvec = [Vec3::default(); POINT_COUNT];
        // initialize random unit vectors
        for i in 0..POINT_COUNT {
            randvec[i] = Vec3::unit_vector(Vec3::random_range(-1.0, 1.0));
        }

        // generate random permutations for x,y,z
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

    pub fn noise(&self, p: &Point3) -> f64 {
        // Find unit cube coordinates surrounding the point
        let u = p.x() - (p.x()).floor();
        let v = p.y() - (p.y()).floor();
        let w = p.z() - (p.z()).floor();

        let i = (p.x().floor()) as i32;
        let j = (p.y().floor()) as i32;
        let k = (p.z().floor()) as i32;

        // Fetch vectors at cube corners, indexed via permutations and hashing
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

    /// Computes turbulence by summing multiple frequencies of noise
    pub fn turb(&self, p: &Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;   // decrease weight by half each octave
            temp_p *= 2.0;   // increase frequency by 2 each octave
        }

        accum.abs()
    }
}

impl Perlin {
    /// Generate a permutation array of 0..255 shuffled randomly
    fn perlin_generate_perm() -> [usize; POINT_COUNT] {
        let mut p = [0usize; POINT_COUNT];
        for i in 0..POINT_COUNT {
            p[i] = i;
        }
        Perlin::permute(&mut p);
        p
    }

    /// Shuffle the array in place using Fisher-Yates algorithm
    fn permute(p: &mut [usize; POINT_COUNT]) {
        for i in (1..POINT_COUNT).rev() {
            let j = i as i32;
            let target = random_int_range(0, j);
            p.swap(i, target as usize);
        }
    }

    /// Performs trilinear interpolation of scalar values at cube corners
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

    /// Interpolates the noise contribution from each corner vector with smoothing
    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        // Hermitian smoothing function to ease interpolation
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
