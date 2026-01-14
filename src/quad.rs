use std::sync::Arc;

use crate::{
    Hittable, Material, Point3, Ray, Vec3,
    aabb::Aabb,
    color::Color,
    hittable::HitRecord,
    hittable_list::HittableList,
    interval::Interval,
    rtweekend::{INFINITY, random_double},
    texture::Texture,
};

/// 2D primitives that can be carved out in the (alpha, beta) plane.
pub enum Primitive {
    Quad,                                        // Unit square 0..=1 x 0..=1
    Disk(f64),                                   // Disk with radius r centered at (0,0)
    Triangle,                                    // Triangle: a>=0, b>=0, a+b <= 1
    Ellipse { rx: f64, ry: f64 },                // Ellipse with radii rx, ry
    Annulus { inner: f64, outer: f64 },          // Ring: inner..outer radius
    TextureMask(Arc<dyn Texture + Send + Sync>), // Mask based on texture value(u,v,p)
    Mandelbrot { iterations: usize },            // Mandelbrot membership (mapped from [0,1]^2)
}

/// Represents a planar quadrilateral (or other primitives) with associated material and geometry.
pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Arc<dyn Material + Send + Sync>,
    bcreate_box: Aabb,
    normal: Vec3,
    d: f64,
    primitive: Primitive,
    area: f64,
}

impl Quad {
    /// Create a new Quad with position `q`, edge vectors `u` and `v`, material `mat`, and shape `primitive`.
    pub fn new(
        q: Point3,
        u: Vec3,
        v: Vec3,
        mat: Arc<dyn Material + Send + Sync>,
        primitive: Primitive,
    ) -> Self {
        let n = Vec3::cross(&u, &v);
        let bcreate_box = Quad::set_bounding_create_box(&q, &u, &v);

        let normal = Vec3::unit_vector(n);
        let d = Vec3::dot(&normal, &q);

        // Vector `w` for barycentric coordinate computations
        let w = n / Vec3::dot(&n, &n);

        let area = n.length();

        Self {
            q,
            u,
            v,
            w,
            mat,
            bcreate_box,
            normal,
            d,
            primitive,
            area,
        }
    }

    /// Compute the bounding create_box containing all four vertices of the quad.
    fn set_bounding_create_box(q: &Point3, u: &Vec3, v: &Vec3) -> Aabb {
        let bcreate_box_diagonal1 = Aabb::new_from_points(*q, *q + *u + *v);
        let bcreate_box_diagonal2 = Aabb::new_from_points(*q + *u, *q + *v);
        Aabb::new_(bcreate_box_diagonal1, bcreate_box_diagonal2)
    }

    /// Returns true if `(a, b)` lies inside the selected primitive shape.
    /// `hit_p` is the 3D intersection point on the plane, used for texture sampling.
    pub fn is_interior(&self, a: f64, b: f64, hit_p: &Point3, rec: &mut HitRecord) -> bool {
        match &self.primitive {
            Primitive::Quad => {
                if (0.0..=1.0).contains(&a) && (0.0..=1.0).contains(&b) {
                    rec.u = a;
                    rec.v = b;
                    true
                } else {
                    false
                }
            }
            Primitive::Disk(r) => {
                let dist2 = a * a + b * b;
                if dist2 <= r * r {
                    rec.u = a;
                    rec.v = b;
                    true
                } else {
                    false
                }
            }
            Primitive::Triangle => {
                if a >= 0.0 && b >= 0.0 && (a + b) <= 1.0 {
                    rec.u = a;
                    rec.v = b;
                    true
                } else {
                    false
                }
            }
            Primitive::Ellipse { rx, ry } => {
                let nx = a / *rx;
                let ny = b / *ry;
                if nx * nx + ny * ny <= 1.0 {
                    rec.u = a;
                    rec.v = b;
                    true
                } else {
                    false
                }
            }
            Primitive::Annulus { inner, outer } => {
                let dist2 = a * a + b * b;
                let inner2 = inner * inner;
                let outer2 = outer * outer;
                if dist2 >= inner2 && dist2 <= outer2 {
                    rec.u = a;
                    rec.v = b;
                    true
                } else {
                    false
                }
            }
            Primitive::TextureMask(tex) => {
                // Quick reject: outside unit square
                if !(0.0..=1.0).contains(&a) || !(0.0..=1.0).contains(&b) {
                    return false;
                }

                // Clamp UV coordinates to [0,1]
                let ua = a.clamp(0.0, 1.0);
                let vb = b.clamp(0.0, 1.0);

                // Sample texture value and compute luminance
                let col: Color = tex.value(ua, vb, hit_p);
                let lum = (col.x() + col.y() + col.z()) / 3.0;

                if lum >= 0.5 {
                    rec.u = a;
                    rec.v = b;
                    true
                } else {
                    false
                }
            }
            Primitive::Mandelbrot { iterations } => {
                // Map (a,b) in [0,1] to complex plane region
                let cre = map_range(a, 0.0, 1.0, -2.0, 1.0);
                let cim = map_range(b, 0.0, 1.0, -1.5, 1.5);

                if mandelbrot_contains(cre, cim, *iterations) {
                    rec.u = a;
                    rec.v = b;
                    true
                } else {
                    false
                }
            }
        }
    }

    /// Create a create_box (six quads) from two opposite corners `a` and `b` with material `mat`.
    pub fn create_box(
        a: &Point3,
        b: &Point3,
        mat: Arc<dyn Material + Send + Sync>,
    ) -> HittableList {
        let mut sides = HittableList::new();

        let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
        let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

        let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

        sides.add(Arc::new(Quad::new(
            Point3::new(min.x(), min.y(), max.z()),
            dx,
            dy,
            mat.clone(),
            Primitive::Quad,
        ))); // front
        sides.add(Arc::new(Quad::new(
            Point3::new(max.x(), min.y(), max.z()),
            -dz,
            dy,
            mat.clone(),
            Primitive::Quad,
        ))); // right
        sides.add(Arc::new(Quad::new(
            Point3::new(max.x(), min.y(), min.z()),
            -dx,
            dy,
            mat.clone(),
            Primitive::Quad,
        ))); // back
        sides.add(Arc::new(Quad::new(
            Point3::new(min.x(), min.y(), min.z()),
            dz,
            dy,
            mat.clone(),
            Primitive::Quad,
        ))); // left
        sides.add(Arc::new(Quad::new(
            Point3::new(min.x(), max.y(), max.z()),
            dx,
            -dz,
            mat.clone(),
            Primitive::Quad,
        ))); // top
        sides.add(Arc::new(Quad::new(
            Point3::new(min.x(), min.y(), min.z()),
            dx,
            dz,
            mat.clone(),
            Primitive::Quad,
        ))); // bottom

        sides
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let denom = Vec3::dot(&self.normal, &r.direction());

        // Return false if ray is parallel to the plane.
        if denom.abs() < 1e-8 {
            return false;
        }

        let t = (self.d - Vec3::dot(&self.normal, &r.origin())) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;

        // Compute planar coordinates alpha, beta
        let alpha = Vec3::dot(&self.w, &Vec3::cross(&planar_hitpt_vector, &self.v));
        let beta = Vec3::dot(&self.w, &Vec3::cross(&self.u, &planar_hitpt_vector));

        if !self.is_interior(alpha, beta, &intersection, rec) {
            return false;
        }

        rec.t = t;
        rec.p = intersection;
        rec.mat = Some(Arc::clone(&self.mat));
        rec.set_face_normal(r, &self.normal);

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bcreate_box
    }

    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        let mut rec = HitRecord::default();

        if !self.hit(
            &Ray::new_no_time(*origin, *direction),
            Interval::new(0.001, INFINITY),
            &mut rec,
        ) {
            return 0.0;
        }
        let distance_squared = rec.t * rec.t * direction.length_squared();
        let cosine = (Vec3::dot(direction, &rec.normal) / direction.length()).abs();

        distance_squared / (cosine * self.area)
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        let p = self.q + (random_double() * self.u) + (random_double() * self.v);
        p - *origin
    }
}

/// Linearly maps `x` from range `[x0, x1]` to `[y0, y1]`.
fn map_range(x: f64, x0: f64, x1: f64, y0: f64, y1: f64) -> f64 {
    let t = if x1 != x0 { (x - x0) / (x1 - x0) } else { 0.0 };
    y0 + t * (y1 - y0)
}

/// Returns true if the point (cre, cim) is in the Mandelbrot set within `iterations` steps.
fn mandelbrot_contains(cre: f64, cim: f64, iterations: usize) -> bool {
    let mut zr = 0.0;
    let mut zi = 0.0;
    let mut zr2 = 0.0;
    let mut zi2 = 0.0;

    for _ in 0..iterations {
        // z = z*z + c where z = zr + i*zi
        zi = 2.0 * zr * zi + cim;
        zr = zr2 - zi2 + cre;

        zr2 = zr * zr;
        zi2 = zi * zi;

        if zr2 + zi2 > 4.0 {
            return false; // escaped -> not in set
        }
    }
    true // did not escape -> inside set
}
