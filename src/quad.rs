use std::sync::Arc;

use crate::{
    Hittable, Material, Point3, Ray, Vec3, aabb::Aabb, color::Color, hittable::HitRecord,
    hittable_list::HittableList, interval::Interval, texture::Texture,
};

/// 2D primitives defined in (alpha, beta) parameter space for carving shapes.
pub enum Primitive {
    Quad,                                        // Unit square [0..=1] x [0..=1]
    Disk(f64),                                   // Disk with radius r centered at origin
    Triangle,                                    // Triangle with a>=0, b>=0, and a+b <= 1
    Ellipse { rx: f64, ry: f64 },                // Ellipse with radii rx, ry
    Annulus { inner: f64, outer: f64 },          // Ring between inner and outer radii
    TextureMask(Arc<dyn Texture + Send + Sync>), // Mask defined by texture luminance
    Mandelbrot { iterations: usize },            // Mandelbrot set membership test
}

/// A planar quad (parallelogram) with an associated primitive shape for hit testing.
pub struct Quad {
    q: Point3,                            // Quad origin point
    u: Vec3,                              // First edge vector
    v: Vec3,                              // Second edge vector
    w: Vec3,                              // Vector for parameter space projection
    mat: Arc<dyn Material + Send + Sync>, // Material of the quad
    bbox: Aabb,                           // Bounding box enclosing the quad
    normal: Vec3,                         // Plane normal vector
    d: f64,                               // Plane offset (dot(normal, q))
    primitive: Primitive,                 // Primitive shape type used for interior testing
}

impl Quad {
    /// Constructs a new Quad with given origin, edges, material, and primitive.
    pub fn new(
        q: Point3,
        u: Vec3,
        v: Vec3,
        mat: Arc<dyn Material + Send + Sync>,
        primitive: Primitive,
    ) -> Self {
        let n = Vec3::cross(&u, &v);
        let bbox = Quad::set_bounding_box(&q, &u, &v);

        let normal = Vec3::unit_vector(n);
        let d = Vec3::dot(&normal, &q);

        // Vector w is n normalized by dot(n,n), used for coordinate projection
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

    /// Computes axis-aligned bounding box enclosing all quad vertices.
    fn set_bounding_box(q: &Point3, u: &Vec3, v: &Vec3) -> Aabb {
        let bbox_diagonal1 = Aabb::new_from_points(*q, *q + *u + *v);
        let bbox_diagonal2 = Aabb::new_from_points(*q + *u, *q + *v);
        Aabb::new_(bbox_diagonal1, bbox_diagonal2)
    }

    /// Returns true if the (a,b) coordinates lie inside the quad's primitive shape.
    /// Updates the hit record's texture coordinates if true.
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
                // Reject if outside unit UV range
                if !(0.0..=1.0).contains(&a) || !(0.0..=1.0).contains(&b) {
                    return false;
                }

                let ua = a.clamp(0.0, 1.0);
                let vb = b.clamp(0.0, 1.0);

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
                // Map (a,b) to complex plane and test membership
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

    /// Constructs a box shape from two points and material, returns six quads forming the box.
    pub fn box_shape(a: &Point3, b: &Point3, mat: Arc<dyn Material + Send + Sync>) -> HittableList {
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

        // Reject ray if parallel to quad plane
        if denom.abs() < 1e-8 {
            return false;
        }

        // Compute intersection distance along ray
        let t = (self.d - Vec3::dot(&self.normal, &r.origin())) / denom;

        // Check if intersection is within ray parameter bounds
        if !ray_t.contains(t) {
            return false;
        }

        let intersection = r.at(t);

        // Compute planar coordinates for hit point relative to quad edges
        let planar_hitpt_vector = intersection - self.q;
        let alpha = Vec3::dot(&self.w, &Vec3::cross(&planar_hitpt_vector, &self.v));
        let beta = Vec3::dot(&self.w, &Vec3::cross(&self.u, &planar_hitpt_vector));

        // Verify hit point lies inside primitive shape
        if !self.is_interior(alpha, beta, &intersection, rec) {
            return false;
        }

        // Update hit record with intersection details
        rec.t = t;
        rec.p = intersection;
        rec.mat = Some(Arc::clone(&self.mat));
        rec.set_face_normal(r, &self.normal);

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

/// Linearly maps `x` from range [x0,x1] to [y0,y1].
fn map_range(x: f64, x0: f64, x1: f64, y0: f64, y1: f64) -> f64 {
    let t = if x1 != x0 { (x - x0) / (x1 - x0) } else { 0.0 };
    y0 + t * (y1 - y0)
}

/// Determines if complex point (cre, cim) belongs to Mandelbrot set for given iterations.
fn mandelbrot_contains(cre: f64, cim: f64, iterations: usize) -> bool {
    let mut zr = 0.0;
    let mut zi = 0.0;
    let mut zr2 = 0.0;
    let mut zi2 = 0.0;

    for _ in 0..iterations {
        zi = 2.0 * zr * zi + cim;
        zr = zr2 - zi2 + cre;

        zr2 = zr * zr;
        zi2 = zi * zi;

        if zr2 + zi2 > 4.0 {
            return false;
        }
    }
    true
}
