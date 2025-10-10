use crate::utils::approx_eq;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn origin() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    #[inline]
    pub fn distance_to(&self, other: &Point3D) -> f64 {
        self.distance_squared_to(other).sqrt()
    }

    #[inline]
    pub fn distance_squared_to(&self, other: &Point3D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }

    #[inline]
    pub fn midpoint(&self, other: &Point3D) -> Point3D {
        Point3D {
            x: (self.x + other.x) * 0.5,
            y: (self.y + other.y) * 0.5,
            z: (self.z + other.z) * 0.5,
        }
    }

    #[inline]
    pub fn translate(&self, vector: &crate::primitives::Vector3D) -> Point3D {
        Point3D {
            x: self.x + vector.x,
            y: self.y + vector.y,
            z: self.z + vector.z,
        }
    }
}

impl PartialEq for Point3D {
    fn eq(&self, other: &Self) -> bool {
        approx_eq(self.x, other.x) && approx_eq(self.y, other.y) && approx_eq(self.z, other.z)
    }
}

impl Add<crate::primitives::Vector3D> for Point3D {
    type Output = Point3D;

    fn add(self, rhs: crate::primitives::Vector3D) -> Self::Output {
        Point3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Point3D> for Point3D {
    type Output = crate::primitives::Vector3D;

    fn sub(self, rhs: Point3D) -> Self::Output {
        crate::primitives::Vector3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_creation() {
        let p = Point3D::new(1.0, 2.0, 3.0);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
        assert_eq!(p.z, 3.0);
    }

    #[test]
    fn test_origin() {
        let p = Point3D::origin();
        assert_eq!(p, Point3D::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_distance() {
        let p1 = Point3D::new(0.0, 0.0, 0.0);
        let p2 = Point3D::new(3.0, 4.0, 0.0);
        assert_eq!(p1.distance_to(&p2), 5.0);
    }

    #[test]
    fn test_midpoint() {
        let p1 = Point3D::new(0.0, 0.0, 0.0);
        let p2 = Point3D::new(2.0, 4.0, 6.0);
        let mid = p1.midpoint(&p2);
        assert_eq!(mid, Point3D::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_equality() {
        let p1 = Point3D::new(1.0, 2.0, 3.0);
        let p2 = Point3D::new(1.0, 2.0, 3.0);
        assert_eq!(p1, p2);
    }
}
