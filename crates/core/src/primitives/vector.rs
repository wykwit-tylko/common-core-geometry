use crate::error::GeometryError;
use crate::utils::{approx_eq, approx_zero};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3D {
    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    #[inline]
    pub fn unit_x() -> Self {
        Self {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    #[inline]
    pub fn unit_y() -> Self {
        Self {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    }

    #[inline]
    pub fn unit_z() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    }

    #[inline]
    pub fn from_points(from: &crate::primitives::Point3D, to: &crate::primitives::Point3D) -> Self {
        Self {
            x: to.x - from.x,
            y: to.y - from.y,
            z: to.z - from.z,
        }
    }

    #[inline]
    pub fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    #[inline]
    pub fn magnitude_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(&self) -> Result<Vector3D, GeometryError> {
        let mag = self.magnitude();
        if approx_zero(mag) {
            return Err(GeometryError::DivisionByZero(
                "Cannot normalize zero vector".to_string(),
            ));
        }
        Ok(Vector3D {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        })
    }

    #[inline]
    pub fn dot(&self, other: &Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline]
    pub fn cross(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn angle(&self, other: &Vector3D) -> f64 {
        let dot = self.dot(other);
        let mag_product = self.magnitude() * other.magnitude();
        if approx_zero(mag_product) {
            return 0.0;
        }
        (dot / mag_product).clamp(-1.0, 1.0).acos()
    }

    pub fn project_onto(&self, other: &Vector3D) -> Vector3D {
        let other_mag_sq = other.magnitude_squared();
        if approx_zero(other_mag_sq) {
            return Vector3D::zero();
        }
        let scalar = self.dot(other) / other_mag_sq;
        *other * scalar
    }

    #[inline]
    pub fn is_parallel(&self, other: &Vector3D) -> bool {
        let cross = self.cross(other);
        approx_zero(cross.magnitude_squared())
    }

    #[inline]
    pub fn is_perpendicular(&self, other: &Vector3D) -> bool {
        approx_zero(self.dot(other))
    }
}

impl PartialEq for Vector3D {
    fn eq(&self, other: &Self) -> bool {
        approx_eq(self.x, other.x) && approx_eq(self.y, other.y) && approx_eq(self.z, other.z)
    }
}

impl Add for Vector3D {
    type Output = Vector3D;

    fn add(self, rhs: Vector3D) -> Self::Output {
        Vector3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector3D {
    type Output = Vector3D;

    fn sub(self, rhs: Vector3D) -> Self::Output {
        Vector3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3D {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vector3D {
    type Output = Vector3D;

    fn div(self, rhs: f64) -> Self::Output {
        Vector3D {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Neg for Vector3D {
    type Output = Vector3D;

    fn neg(self) -> Self::Output {
        Vector3D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_creation() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_unit_vectors() {
        assert_eq!(Vector3D::unit_x(), Vector3D::new(1.0, 0.0, 0.0));
        assert_eq!(Vector3D::unit_y(), Vector3D::new(0.0, 1.0, 0.0));
        assert_eq!(Vector3D::unit_z(), Vector3D::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_magnitude() {
        let v = Vector3D::new(3.0, 4.0, 0.0);
        assert_eq!(v.magnitude(), 5.0);
    }

    #[test]
    fn test_normalize() {
        let v = Vector3D::new(3.0, 4.0, 0.0);
        let n = v.normalize().unwrap();
        assert!((n.magnitude() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_dot_product() {
        let v1 = Vector3D::new(1.0, 0.0, 0.0);
        let v2 = Vector3D::new(0.0, 1.0, 0.0);
        assert_eq!(v1.dot(&v2), 0.0);
    }

    #[test]
    fn test_cross_product() {
        let v1 = Vector3D::new(1.0, 0.0, 0.0);
        let v2 = Vector3D::new(0.0, 1.0, 0.0);
        let cross = v1.cross(&v2);
        assert_eq!(cross, Vector3D::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_parallel() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(2.0, 4.0, 6.0);
        assert!(v1.is_parallel(&v2));
    }

    #[test]
    fn test_perpendicular() {
        let v1 = Vector3D::new(1.0, 0.0, 0.0);
        let v2 = Vector3D::new(0.0, 1.0, 0.0);
        assert!(v1.is_perpendicular(&v2));
    }
}
