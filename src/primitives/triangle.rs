use crate::error::GeometryError;
use crate::primitives::{Plane, Point3D, Vector3D, AABB};
use crate::utils::approx_zero;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub a: Point3D,
    pub b: Point3D,
    pub c: Point3D,
}

impl Triangle {
    pub fn new(a: Point3D, b: Point3D, c: Point3D) -> Result<Self, GeometryError> {
        let v1 = Vector3D::from_points(&a, &b);
        let v2 = Vector3D::from_points(&a, &c);
        let cross = v1.cross(&v2);

        if approx_zero(cross.magnitude_squared()) {
            return Err(GeometryError::DegenerateCase(
                "Triangle vertices are collinear".to_string(),
            ));
        }

        Ok(Self { a, b, c })
    }

    #[inline]
    pub fn normal(&self) -> Vector3D {
        let v1 = Vector3D::from_points(&self.a, &self.b);
        let v2 = Vector3D::from_points(&self.a, &self.c);
        v1.cross(&v2).normalize().unwrap_or(Vector3D::unit_z())
    }

    #[inline]
    pub fn area(&self) -> f64 {
        let v1 = Vector3D::from_points(&self.a, &self.b);
        let v2 = Vector3D::from_points(&self.a, &self.c);
        v1.cross(&v2).magnitude() * 0.5
    }

    #[inline]
    pub fn centroid(&self) -> Point3D {
        Point3D::new(
            (self.a.x + self.b.x + self.c.x) / 3.0,
            (self.a.y + self.b.y + self.c.y) / 3.0,
            (self.a.z + self.b.z + self.c.z) / 3.0,
        )
    }

    pub fn to_plane(&self) -> Plane {
        Plane::from_three_points(&self.a, &self.b, &self.c)
            .expect("Triangle was already validated as non-collinear")
    }

    pub fn bounding_box(&self) -> AABB {
        let min = Point3D::new(
            self.a.x.min(self.b.x).min(self.c.x),
            self.a.y.min(self.b.y).min(self.c.y),
            self.a.z.min(self.b.z).min(self.c.z),
        );
        let max = Point3D::new(
            self.a.x.max(self.b.x).max(self.c.x),
            self.a.y.max(self.b.y).max(self.c.y),
            self.a.z.max(self.b.z).max(self.c.z),
        );
        AABB::new(min, max).unwrap_or_else(|_| {
            AABB::new(
                Point3D::new(min.x - 0.001, min.y - 0.001, min.z - 0.001),
                Point3D::new(max.x + 0.001, max.y + 0.001, max.z + 0.001),
            )
            .unwrap()
        })
    }

    pub fn barycentric_coords(&self, point: &Point3D) -> (f64, f64, f64) {
        let v0 = Vector3D::from_points(&self.a, &self.b);
        let v1 = Vector3D::from_points(&self.a, &self.c);
        let v2 = Vector3D::from_points(&self.a, point);

        let d00 = v0.dot(&v0);
        let d01 = v0.dot(&v1);
        let d11 = v1.dot(&v1);
        let d20 = v2.dot(&v0);
        let d21 = v2.dot(&v1);

        let denom = d00 * d11 - d01 * d01;

        if approx_zero(denom) {
            return (1.0, 0.0, 0.0);
        }

        let v = (d11 * d20 - d01 * d21) / denom;
        let w = (d00 * d21 - d01 * d20) / denom;
        let u = 1.0 - v - w;

        (u, v, w)
    }

    pub fn contains_point(&self, point: &Point3D) -> bool {
        let plane = self.to_plane();
        if !plane.contains_point(point) {
            return false;
        }

        let (u, v, w) = self.barycentric_coords(point);
        u >= 0.0 && v >= 0.0 && w >= 0.0
    }
}

impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b && self.c == other.c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle_creation() {
        let tri = Triangle::new(
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
            Point3D::new(0.0, 1.0, 0.0),
        )
        .unwrap();
        assert_eq!(tri.a, Point3D::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_collinear_triangle() {
        let result = Triangle::new(
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
            Point3D::new(2.0, 0.0, 0.0),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_area() {
        let tri = Triangle::new(
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(2.0, 0.0, 0.0),
            Point3D::new(0.0, 2.0, 0.0),
        )
        .unwrap();
        assert_eq!(tri.area(), 2.0);
    }

    #[test]
    fn test_centroid() {
        let tri = Triangle::new(
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(3.0, 0.0, 0.0),
            Point3D::new(0.0, 3.0, 0.0),
        )
        .unwrap();
        assert_eq!(tri.centroid(), Point3D::new(1.0, 1.0, 0.0));
    }
}
