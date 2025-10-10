use crate::error::GeometryError;
use crate::primitives::{Point3D, Vector3D};
use crate::utils::clamp;

#[derive(Debug, Clone, Copy)]
pub struct LineSegment {
    pub start: Point3D,
    pub end: Point3D,
}

impl LineSegment {
    pub fn new(start: Point3D, end: Point3D) -> Result<Self, GeometryError> {
        if start == end {
            return Err(GeometryError::DegenerateCase(
                "Line segment start and end points must be different".to_string(),
            ));
        }
        Ok(Self { start, end })
    }

    #[inline]
    pub fn length(&self) -> f64 {
        self.start.distance_to(&self.end)
    }

    #[inline]
    pub fn direction(&self) -> Vector3D {
        Vector3D::from_points(&self.start, &self.end)
    }

    #[inline]
    pub fn midpoint(&self) -> Point3D {
        self.start.midpoint(&self.end)
    }

    #[inline]
    pub fn point_at(&self, t: f64) -> Point3D {
        let dir = self.direction();
        self.start + dir * t
    }

    pub fn closest_point(&self, point: &Point3D) -> Point3D {
        let dir = self.direction();
        let to_point = Vector3D::from_points(&self.start, point);
        let length_sq = dir.magnitude_squared();

        let t = if length_sq > 0.0 {
            to_point.dot(&dir) / length_sq
        } else {
            0.0
        };

        let clamped_t = clamp(t, 0.0, 1.0);
        self.point_at(clamped_t)
    }

    #[inline]
    pub fn distance_to_point(&self, point: &Point3D) -> f64 {
        let closest = self.closest_point(point);
        point.distance_to(&closest)
    }
}

impl PartialEq for LineSegment {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_segment_creation() {
        let start = Point3D::new(0.0, 0.0, 0.0);
        let end = Point3D::new(1.0, 0.0, 0.0);
        let seg = LineSegment::new(start, end).unwrap();
        assert_eq!(seg.start, start);
        assert_eq!(seg.end, end);
    }

    #[test]
    fn test_degenerate_line_segment() {
        let p = Point3D::new(0.0, 0.0, 0.0);
        assert!(LineSegment::new(p, p).is_err());
    }

    #[test]
    fn test_length() {
        let seg =
            LineSegment::new(Point3D::new(0.0, 0.0, 0.0), Point3D::new(3.0, 4.0, 0.0)).unwrap();
        assert_eq!(seg.length(), 5.0);
    }

    #[test]
    fn test_midpoint() {
        let seg =
            LineSegment::new(Point3D::new(0.0, 0.0, 0.0), Point3D::new(2.0, 2.0, 2.0)).unwrap();
        assert_eq!(seg.midpoint(), Point3D::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_point_at() {
        let seg =
            LineSegment::new(Point3D::new(0.0, 0.0, 0.0), Point3D::new(10.0, 0.0, 0.0)).unwrap();
        assert_eq!(seg.point_at(0.5), Point3D::new(5.0, 0.0, 0.0));
    }
}
