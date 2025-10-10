use std::f64::consts::PI;

use crate::error::GeometryError;
use crate::primitives::Point3D;
use crate::utils::approx_eq;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Point3D,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3D, radius: f64) -> Result<Self, GeometryError> {
        if radius <= 0.0 {
            return Err(GeometryError::InvalidConstruction(
                "Sphere radius must be positive".to_string(),
            ));
        }
        Ok(Self { center, radius })
    }

    #[inline]
    pub fn volume(&self) -> f64 {
        (4.0 / 3.0) * PI * self.radius.powi(3)
    }

    #[inline]
    pub fn surface_area(&self) -> f64 {
        4.0 * PI * self.radius.powi(2)
    }

    #[inline]
    pub fn contains_point(&self, point: &Point3D) -> bool {
        self.center.distance_squared_to(point) <= self.radius * self.radius
    }

    #[inline]
    pub fn distance_to_point(&self, point: &Point3D) -> f64 {
        (self.center.distance_to(point) - self.radius).max(0.0)
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center && approx_eq(self.radius, other.radius)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sphere_creation() {
        let sphere = Sphere::new(Point3D::origin(), 5.0).unwrap();
        assert_eq!(sphere.center, Point3D::origin());
        assert_eq!(sphere.radius, 5.0);
    }

    #[test]
    fn test_invalid_radius() {
        assert!(Sphere::new(Point3D::origin(), 0.0).is_err());
        assert!(Sphere::new(Point3D::origin(), -1.0).is_err());
    }

    #[test]
    fn test_volume() {
        let sphere = Sphere::new(Point3D::origin(), 1.0).unwrap();
        let expected = (4.0 / 3.0) * PI;
        assert!((sphere.volume() - expected).abs() < 1e-10);
    }

    #[test]
    fn test_surface_area() {
        let sphere = Sphere::new(Point3D::origin(), 1.0).unwrap();
        let expected = 4.0 * PI;
        assert!((sphere.surface_area() - expected).abs() < 1e-10);
    }

    #[test]
    fn test_contains_point() {
        let sphere = Sphere::new(Point3D::origin(), 5.0).unwrap();
        assert!(sphere.contains_point(&Point3D::new(3.0, 0.0, 0.0)));
        assert!(!sphere.contains_point(&Point3D::new(6.0, 0.0, 0.0)));
    }
}
