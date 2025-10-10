use crate::error::Result;
use crate::primitives::{Point3D, Vector3D};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3D,
    pub direction: Vector3D,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Vector3D) -> Result<Self> {
        let normalized_direction = direction.normalize()?;
        Ok(Self {
            origin,
            direction: normalized_direction,
        })
    }

    #[inline]
    pub fn point_at(&self, t: f64) -> Point3D {
        Point3D {
            x: self.origin.x + self.direction.x * t,
            y: self.origin.y + self.direction.y * t,
            z: self.origin.z + self.direction.z * t,
        }
    }
}

impl PartialEq for Ray {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.direction == other.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::approx_eq;

    #[test]
    fn test_ray_creation() {
        let origin = Point3D::new(0.0, 0.0, 0.0);
        let direction = Vector3D::new(1.0, 0.0, 0.0);
        let ray = Ray::new(origin, direction).unwrap();

        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, Vector3D::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_ray_normalizes_direction() {
        let origin = Point3D::new(0.0, 0.0, 0.0);
        let direction = Vector3D::new(3.0, 4.0, 0.0);
        let ray = Ray::new(origin, direction).unwrap();

        assert_eq!(ray.direction.magnitude(), 1.0);
        assert!(approx_eq(ray.direction.x, 0.6));
        assert!(approx_eq(ray.direction.y, 0.8));
    }

    #[test]
    fn test_ray_point_at() {
        let origin = Point3D::new(1.0, 2.0, 3.0);
        let direction = Vector3D::new(1.0, 0.0, 0.0);
        let ray = Ray::new(origin, direction).unwrap();

        let p0 = ray.point_at(0.0);
        assert_eq!(p0, Point3D::new(1.0, 2.0, 3.0));

        let p5 = ray.point_at(5.0);
        assert_eq!(p5, Point3D::new(6.0, 2.0, 3.0));

        let p_neg = ray.point_at(-2.0);
        assert_eq!(p_neg, Point3D::new(-1.0, 2.0, 3.0));
    }

    #[test]
    fn test_ray_zero_direction_fails() {
        let origin = Point3D::new(0.0, 0.0, 0.0);
        let direction = Vector3D::new(0.0, 0.0, 0.0);
        let result = Ray::new(origin, direction);

        assert!(result.is_err());
    }

    #[test]
    fn test_ray_equality() {
        let ray1 = Ray::new(Point3D::new(0.0, 0.0, 0.0), Vector3D::new(1.0, 0.0, 0.0)).unwrap();
        let ray2 = Ray::new(Point3D::new(0.0, 0.0, 0.0), Vector3D::new(2.0, 0.0, 0.0)).unwrap();
        assert_eq!(ray1, ray2);
    }
}
