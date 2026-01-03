use std::rc::Rc;

use crate::{
    Hittable, Material, Point3, Ray, Vec3, aabb::AABB, color::Color, hittable::HitRecord, hittable_list::{self, HittableList}, interval::Interval, texture::Texture
};

/// 2D primitives that can be carved out in the (alpha, beta) plane.
pub enum Primitive {
    Quad,                               // unit square 0..=1 x 0..=1
    Disk(f64),                          // disk radius r centered at (0,0)
    Triangle,                           // triangle: a>=0, b>=0, a+b <= 1
    Ellipse { rx: f64, ry: f64 },       // ellipse with radii rx, ry
    Annulus { inner: f64, outer: f64 }, // ring: inner..outer
    TextureMask(Rc<dyn Texture>),       // mask based on texture value(u,v,p)
    Mandelbrot { iterations: usize },   // mandelbrot membership (mapped from [0,1]^2)
}

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Rc<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    d: f64,
    primitive: Primitive,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Rc<dyn Material>, primitive: Primitive) -> Self {
        let n = Vec3::cross(&u, &v);
        let bbox = Quad::set_bounding_box(&q, &u, &v);

        let normal = Vec3::unit_vector(n);
        let d = Vec3::dot(&normal, &q);

        // matches your previous w: n / dot(n,n)
        let w = n / Vec3::dot(&n, &n);

        Self {
            q,
            u,
            v,
            w,
            mat,
            bbox,
            normal,
            d,
            primitive,
        }
    }

    fn set_bounding_box(q: &Point3, u: &Vec3, v: &Vec3) -> AABB {
        // Compute the bounding box of all four vertices.
        let bbox_diagonal1 = AABB::new_from_points(*q, *q + *u + *v);
        let bbox_diagonal2 = AABB::new_from_points(*q + *u, *q + *v);
        AABB::new_(bbox_diagonal1, bbox_diagonal2)
    }

    /// Return true iff (a,b) is inside the chosen primitive.
    /// `hit_p` is the 3D intersection point on the plane (needed for texture sampling).
    pub fn is_interior(&self, a: f64, b: f64, hit_p: &Point3, rec: &mut HitRecord) -> bool {
        match &self.primitive {
            Primitive::Quad => {
                if a >= 0.0 && a <= 1.0 && b >= 0.0 && b <= 1.0 {
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
                // quick reject: outside unit square
                if a < 0.0 || a > 1.0 || b < 0.0 || b > 1.0 {
                    return false;
                }

                // Use (a,b) directly as UV; clamp for safety
                let ua = a.clamp(0.0, 1.0);
                let vb = b.clamp(0.0, 1.0);

                // Sample the texture using your project's Texture trait:
                let col: Color = tex.value(ua, vb, hit_p);

                // Compute luminance / brightness to decide mask.
                // I assume Color exposes component accessors x(), y(), z().
                // If your Color type uses r()/g()/b() or other names, replace accordingly.
                let lum = (col.x() + col.y() + col.z()) / 3.0;

                // Threshold can be tuned
                if lum >= 0.5 {
                    rec.u = a;
                    rec.v = b;
                    true
                } else {
                    false
                }
            }

            Primitive::Mandelbrot { iterations } => {
                // map (a,b) in [0,1] to complex plane region (adjust domain if you want different view)
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

    pub fn Box(a: &Point3, b: &Point3, mat: Rc<dyn Material>) -> HittableList {
       
        // Returns the 3D box (six sides) that contains the two opposite vertices a & b.

        let mut sides = HittableList::new();

        // Construct the two opposite vertices with the minimum and maximum coordinates.
        let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()),);
        let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()),);

        let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

        sides.add(Rc::new(Quad::new(Point3::new(min.x(), min.y(), max.z()),  dx,  dy, mat.clone(), Primitive::Quad)));  // front
        sides.add(Rc::new(Quad::new(Point3::new(max.x(), min.y(), max.z()), -dz,  dy, mat.clone(), Primitive::Quad)));  // right
        sides.add(Rc::new(Quad::new(Point3::new(max.x(), min.y(), min.z()), -dx,  dy, mat.clone(), Primitive::Quad)));  // back
        sides.add(Rc::new(Quad::new(Point3::new(min.x(), min.y(), min.z()),  dz,  dy, mat.clone(), Primitive::Quad)));  // left
        sides.add(Rc::new(Quad::new(Point3::new(min.x(), max.y(), max.z()),  dx, -dz, mat.clone(), Primitive::Quad)));  // top
        sides.add(Rc::new(Quad::new(Point3::new(min.x(), min.y(), min.z()),  dx,  dz, mat.clone(), Primitive::Quad)));  // bottom

        return sides;
}


}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let denom = Vec3::dot(&self.normal, &r.direction());

        // No hit if the ray is parallel to the plane.
        if denom.abs() < 1e-8 {
            return false;
        }

        // Return false if the hit point parameter t is outside the ray interval.
        let t = (self.d - Vec3::dot(&self.normal, &r.origin())) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        // Determine if the hit point lies within the planar shape using its plane coordinates.
        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = Vec3::dot(&self.w, &Vec3::cross(&planar_hitpt_vector, &self.v));
        let beta = Vec3::dot(&self.w, &Vec3::cross(&self.u, &planar_hitpt_vector));

        // pass intersection to is_interior so texture sampling has access to 3D hit point
        if !self.is_interior(alpha, beta, &intersection, rec) {
            return false;
        }

        // Ray hits the 2D shape; set the rest of the hit record and return true.
        rec.t = t;
        rec.p = intersection;
        rec.mat = Some(Rc::clone(&self.mat));
        rec.set_face_normal(r, &self.normal);

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

/// Utility: linear mapping
fn map_range(x: f64, x0: f64, x1: f64, y0: f64, y1: f64) -> f64 {
    let t = if x1 != x0 { (x - x0) / (x1 - x0) } else { 0.0 };
    y0 + t * (y1 - y0)
}

/// Mandelbrot escape-time membership: true if point does NOT escape within `iterations`.
fn mandelbrot_contains(cre: f64, cim: f64, iterations: usize) -> bool {
    let mut zr = 0.0;
    let mut zi = 0.0;
    let mut zr2 = 0.0;
    let mut zi2 = 0.0;

    for _ in 0..iterations {
        // z = z*z + c  where z = zr + i*zi
        zi = 2.0 * zr * zi + cim;
        zr = zr2 - zi2 + cre;

        zr2 = zr * zr;
        zi2 = zi * zi;

        if zr2 + zi2 > 4.0 {
            return false; // escaped -> not in set
        }
    }
    true // did not escape -> treat as interior
}
