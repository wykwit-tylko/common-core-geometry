use crate::primitives::{LineSegment, Point3D, Sphere, Triangle, Vector3D, AABB};

pub trait Transformable {
    fn translate(&self, v: &Vector3D) -> Self;
    fn scale(&self, center: &Point3D, factor: f64) -> Self;
}

impl Transformable for Point3D {
    #[inline]
    fn translate(&self, v: &Vector3D) -> Self {
        Point3D::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }

    #[inline]
    fn scale(&self, center: &Point3D, factor: f64) -> Self {
        let offset = *self - *center;
        *center + offset * factor
    }
}

impl Transformable for LineSegment {
    #[inline]
    fn translate(&self, v: &Vector3D) -> Self {
        LineSegment::new(self.start.translate(v), self.end.translate(v))
            .expect("Translation preserves non-degeneracy")
    }

    #[inline]
    fn scale(&self, center: &Point3D, factor: f64) -> Self {
        LineSegment::new(
            self.start.scale(center, factor),
            self.end.scale(center, factor),
        )
        .expect("Scaling preserves non-degeneracy")
    }
}

impl Transformable for Sphere {
    #[inline]
    fn translate(&self, v: &Vector3D) -> Self {
        Sphere::new(self.center.translate(v), self.radius)
            .expect("Translation preserves positive radius")
    }

    #[inline]
    fn scale(&self, center: &Point3D, factor: f64) -> Self {
        Sphere::new(
            self.center.scale(center, factor),
            self.radius * factor.abs(),
        )
        .expect("Scaling preserves positive radius")
    }
}

impl Transformable for AABB {
    #[inline]
    fn translate(&self, v: &Vector3D) -> Self {
        AABB::new(self.min.translate(v), self.max.translate(v))
            .expect("Translation preserves AABB validity")
    }

    #[inline]
    fn scale(&self, center: &Point3D, factor: f64) -> Self {
        let scaled_min = self.min.scale(center, factor);
        let scaled_max = self.max.scale(center, factor);

        let actual_min = Point3D::new(
            scaled_min.x.min(scaled_max.x),
            scaled_min.y.min(scaled_max.y),
            scaled_min.z.min(scaled_max.z),
        );

        let actual_max = Point3D::new(
            scaled_min.x.max(scaled_max.x),
            scaled_min.y.max(scaled_max.y),
            scaled_min.z.max(scaled_max.z),
        );

        AABB::new(actual_min, actual_max).expect("Scaling preserves AABB validity")
    }
}

impl Transformable for Triangle {
    #[inline]
    fn translate(&self, v: &Vector3D) -> Self {
        Triangle::new(
            self.a.translate(v),
            self.b.translate(v),
            self.c.translate(v),
        )
        .expect("Translation preserves non-collinearity")
    }

    #[inline]
    fn scale(&self, center: &Point3D, factor: f64) -> Self {
        Triangle::new(
            self.a.scale(center, factor),
            self.b.scale(center, factor),
            self.c.scale(center, factor),
        )
        .expect("Scaling preserves non-collinearity")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::approx_eq;

    #[test]
    fn test_point_translate() {
        let p = Point3D::new(1.0, 2.0, 3.0);
        let v = Vector3D::new(1.0, 1.0, 1.0);
        let result = p.translate(&v);

        assert!(approx_eq(result.x, 2.0));
        assert!(approx_eq(result.y, 3.0));
        assert!(approx_eq(result.z, 4.0));
    }

    #[test]
    fn test_point_scale() {
        let p = Point3D::new(2.0, 4.0, 6.0);
        let center = Point3D::origin();
        let result = p.scale(&center, 2.0);

        assert!(approx_eq(result.x, 4.0));
        assert!(approx_eq(result.y, 8.0));
        assert!(approx_eq(result.z, 12.0));
    }

    #[test]
    fn test_point_scale_from_center() {
        let p = Point3D::new(3.0, 3.0, 3.0);
        let center = Point3D::new(1.0, 1.0, 1.0);
        let result = p.scale(&center, 2.0);

        assert!(approx_eq(result.x, 5.0));
        assert!(approx_eq(result.y, 5.0));
        assert!(approx_eq(result.z, 5.0));
    }

    #[test]
    fn test_line_segment_translate() {
        let seg = LineSegment::new(Point3D::origin(), Point3D::new(1.0, 0.0, 0.0)).unwrap();
        let v = Vector3D::new(1.0, 2.0, 3.0);
        let result = seg.translate(&v);

        assert_eq!(result.start, Point3D::new(1.0, 2.0, 3.0));
        assert_eq!(result.end, Point3D::new(2.0, 2.0, 3.0));
    }

    #[test]
    fn test_line_segment_scale() {
        let seg =
            LineSegment::new(Point3D::new(1.0, 0.0, 0.0), Point3D::new(2.0, 0.0, 0.0)).unwrap();
        let center = Point3D::origin();
        let result = seg.scale(&center, 2.0);

        assert_eq!(result.start, Point3D::new(2.0, 0.0, 0.0));
        assert_eq!(result.end, Point3D::new(4.0, 0.0, 0.0));
    }

    #[test]
    fn test_sphere_translate() {
        let sphere = Sphere::new(Point3D::origin(), 1.0).unwrap();
        let v = Vector3D::new(5.0, 5.0, 5.0);
        let result = sphere.translate(&v);

        assert_eq!(result.center, Point3D::new(5.0, 5.0, 5.0));
        assert!(approx_eq(result.radius, 1.0));
    }

    #[test]
    fn test_sphere_scale() {
        let sphere = Sphere::new(Point3D::new(1.0, 0.0, 0.0), 2.0).unwrap();
        let center = Point3D::origin();
        let result = sphere.scale(&center, 3.0);

        assert_eq!(result.center, Point3D::new(3.0, 0.0, 0.0));
        assert!(approx_eq(result.radius, 6.0));
    }

    #[test]
    fn test_aabb_translate() {
        let aabb = AABB::new(Point3D::origin(), Point3D::new(1.0, 1.0, 1.0)).unwrap();
        let v = Vector3D::new(2.0, 2.0, 2.0);
        let result = aabb.translate(&v);

        assert_eq!(result.min, Point3D::new(2.0, 2.0, 2.0));
        assert_eq!(result.max, Point3D::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn test_aabb_scale_positive() {
        let aabb = AABB::new(Point3D::new(1.0, 1.0, 1.0), Point3D::new(2.0, 2.0, 2.0)).unwrap();
        let center = Point3D::origin();
        let result = aabb.scale(&center, 2.0);

        assert_eq!(result.min, Point3D::new(2.0, 2.0, 2.0));
        assert_eq!(result.max, Point3D::new(4.0, 4.0, 4.0));
    }

    #[test]
    fn test_aabb_scale_negative() {
        let aabb = AABB::new(Point3D::new(1.0, 1.0, 1.0), Point3D::new(2.0, 2.0, 2.0)).unwrap();
        let center = Point3D::origin();
        let result = aabb.scale(&center, -1.0);

        assert_eq!(result.min, Point3D::new(-2.0, -2.0, -2.0));
        assert_eq!(result.max, Point3D::new(-1.0, -1.0, -1.0));
    }

    #[test]
    fn test_triangle_translate() {
        let tri = Triangle::new(
            Point3D::origin(),
            Point3D::new(1.0, 0.0, 0.0),
            Point3D::new(0.0, 1.0, 0.0),
        )
        .unwrap();
        let v = Vector3D::new(1.0, 1.0, 1.0);
        let result = tri.translate(&v);

        assert_eq!(result.a, Point3D::new(1.0, 1.0, 1.0));
        assert_eq!(result.b, Point3D::new(2.0, 1.0, 1.0));
        assert_eq!(result.c, Point3D::new(1.0, 2.0, 1.0));
    }

    #[test]
    fn test_triangle_scale() {
        let tri = Triangle::new(
            Point3D::new(1.0, 0.0, 0.0),
            Point3D::new(2.0, 0.0, 0.0),
            Point3D::new(1.0, 1.0, 0.0),
        )
        .unwrap();
        let center = Point3D::origin();
        let result = tri.scale(&center, 2.0);

        assert_eq!(result.a, Point3D::new(2.0, 0.0, 0.0));
        assert_eq!(result.b, Point3D::new(4.0, 0.0, 0.0));
        assert_eq!(result.c, Point3D::new(2.0, 2.0, 0.0));
    }
}
