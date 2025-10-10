use crate::error::GeometryError;
use crate::primitives::{Point3D, Vector3D};

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    pub min: Point3D,
    pub max: Point3D,
}

impl AABB {
    pub fn new(min: Point3D, max: Point3D) -> Result<Self, GeometryError> {
        if min.x >= max.x || min.y >= max.y || min.z >= max.z {
            return Err(GeometryError::InvalidConstruction(
                "AABB min must be less than max in all dimensions".to_string(),
            ));
        }
        Ok(Self { min, max })
    }

    pub fn from_points(points: &[Point3D]) -> Result<Self, GeometryError> {
        if points.is_empty() {
            return Err(GeometryError::InvalidParameter(
                "Cannot create AABB from empty point list".to_string(),
            ));
        }

        let mut min = points[0];
        let mut max = points[0];

        for point in points.iter().skip(1) {
            min.x = min.x.min(point.x);
            min.y = min.y.min(point.y);
            min.z = min.z.min(point.z);
            max.x = max.x.max(point.x);
            max.y = max.y.max(point.y);
            max.z = max.z.max(point.z);
        }

        if min == max {
            return Err(GeometryError::DegenerateCase(
                "All points are identical, cannot create AABB with volume".to_string(),
            ));
        }

        Ok(Self { min, max })
    }

    #[inline]
    pub fn center(&self) -> Point3D {
        self.min.midpoint(&self.max)
    }

    #[inline]
    pub fn size(&self) -> Vector3D {
        Vector3D::from_points(&self.min, &self.max)
    }

    #[inline]
    pub fn volume(&self) -> f64 {
        let size = self.size();
        size.x * size.y * size.z
    }

    #[inline]
    pub fn surface_area(&self) -> f64 {
        let size = self.size();
        2.0 * (size.x * size.y + size.y * size.z + size.z * size.x)
    }

    #[inline]
    pub fn diagonal(&self) -> f64 {
        self.min.distance_to(&self.max)
    }

    #[inline]
    pub fn contains_point(&self, point: &Point3D) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
            && point.z >= self.min.z
            && point.z <= self.max.z
    }

    #[inline]
    pub fn intersects(&self, other: &AABB) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }

    #[inline]
    pub fn union(&self, other: &AABB) -> AABB {
        AABB {
            min: Point3D::new(
                self.min.x.min(other.min.x),
                self.min.y.min(other.min.y),
                self.min.z.min(other.min.z),
            ),
            max: Point3D::new(
                self.max.x.max(other.max.x),
                self.max.y.max(other.max.y),
                self.max.z.max(other.max.z),
            ),
        }
    }

    #[inline]
    pub fn expand_by_point(&self, point: &Point3D) -> AABB {
        AABB {
            min: Point3D::new(
                self.min.x.min(point.x),
                self.min.y.min(point.y),
                self.min.z.min(point.z),
            ),
            max: Point3D::new(
                self.max.x.max(point.x),
                self.max.y.max(point.y),
                self.max.z.max(point.z),
            ),
        }
    }

    #[inline]
    pub fn expand_by_scalar(&self, amount: f64) -> AABB {
        AABB {
            min: Point3D::new(
                self.min.x - amount,
                self.min.y - amount,
                self.min.z - amount,
            ),
            max: Point3D::new(
                self.max.x + amount,
                self.max.y + amount,
                self.max.z + amount,
            ),
        }
    }
}

impl PartialEq for AABB {
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min && self.max == other.max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aabb_creation() {
        let aabb = AABB::new(Point3D::new(0.0, 0.0, 0.0), Point3D::new(1.0, 1.0, 1.0)).unwrap();
        assert_eq!(aabb.min, Point3D::new(0.0, 0.0, 0.0));
        assert_eq!(aabb.max, Point3D::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_invalid_aabb() {
        assert!(AABB::new(Point3D::new(1.0, 0.0, 0.0), Point3D::new(0.0, 1.0, 1.0)).is_err());
    }

    #[test]
    fn test_from_points() {
        let points = vec![
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(1.0, 2.0, 3.0),
            Point3D::new(-1.0, 1.0, 2.0),
        ];
        let aabb = AABB::from_points(&points).unwrap();
        assert_eq!(aabb.min, Point3D::new(-1.0, 0.0, 0.0));
        assert_eq!(aabb.max, Point3D::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_center() {
        let aabb = AABB::new(Point3D::new(0.0, 0.0, 0.0), Point3D::new(2.0, 2.0, 2.0)).unwrap();
        assert_eq!(aabb.center(), Point3D::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_volume() {
        let aabb = AABB::new(Point3D::new(0.0, 0.0, 0.0), Point3D::new(2.0, 3.0, 4.0)).unwrap();
        assert_eq!(aabb.volume(), 24.0);
    }

    #[test]
    fn test_contains_point() {
        let aabb = AABB::new(Point3D::new(0.0, 0.0, 0.0), Point3D::new(1.0, 1.0, 1.0)).unwrap();
        assert!(aabb.contains_point(&Point3D::new(0.5, 0.5, 0.5)));
        assert!(!aabb.contains_point(&Point3D::new(2.0, 0.5, 0.5)));
    }

    #[test]
    fn test_intersects() {
        let aabb1 = AABB::new(Point3D::new(0.0, 0.0, 0.0), Point3D::new(1.0, 1.0, 1.0)).unwrap();
        let aabb2 = AABB::new(Point3D::new(0.5, 0.5, 0.5), Point3D::new(1.5, 1.5, 1.5)).unwrap();
        let aabb3 = AABB::new(Point3D::new(2.0, 0.0, 0.0), Point3D::new(3.0, 1.0, 1.0)).unwrap();
        assert!(aabb1.intersects(&aabb2));
        assert!(!aabb1.intersects(&aabb3));
    }
}
