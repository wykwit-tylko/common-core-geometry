use crate::error::GeometryError;
use crate::primitives::{Point3D, Vector3D};
use crate::utils::{approx_eq, approx_zero};

#[derive(Debug, Clone, Copy)]
pub struct Plane {
    pub normal: Vector3D,
    pub d: f64,
}

impl Plane {
    pub fn new(normal: Vector3D, d: f64) -> Result<Self, GeometryError> {
        let normalized = normal.normalize()?;
        Ok(Self {
            normal: normalized,
            d,
        })
    }

    pub fn from_point_normal(point: &Point3D, normal: &Vector3D) -> Result<Self, GeometryError> {
        let normalized = normal.normalize()?;
        let d = -(normalized.x * point.x + normalized.y * point.y + normalized.z * point.z);
        Ok(Self {
            normal: normalized,
            d,
        })
    }

    pub fn from_three_points(
        p1: &Point3D,
        p2: &Point3D,
        p3: &Point3D,
    ) -> Result<Self, GeometryError> {
        let v1 = Vector3D::from_points(p1, p2);
        let v2 = Vector3D::from_points(p1, p3);
        let normal = v1.cross(&v2);
        
        if approx_zero(normal.magnitude_squared()) {
            return Err(GeometryError::DegenerateCase(
                "Points are collinear, cannot define a unique plane".to_string(),
            ));
        }
        
        Self::from_point_normal(p1, &normal)
    }

    #[inline]
    pub fn distance_to_point(&self, point: &Point3D) -> f64 {
        self.normal.x * point.x + self.normal.y * point.y + self.normal.z * point.z + self.d
    }

    #[inline]
    pub fn closest_point(&self, point: &Point3D) -> Point3D {
        let dist = self.distance_to_point(point);
        let offset = self.normal * dist;
        *point + (offset * -1.0)
    }

    #[inline]
    pub fn contains_point(&self, point: &Point3D) -> bool {
        approx_zero(self.distance_to_point(point))
    }

    #[inline]
    pub fn flip_normal(&self) -> Plane {
        Plane {
            normal: -self.normal,
            d: -self.d,
        }
    }

    #[inline]
    pub fn is_parallel(&self, other: &Plane) -> bool {
        self.normal.is_parallel(&other.normal)
    }
}

impl PartialEq for Plane {
    fn eq(&self, other: &Self) -> bool {
        self.normal == other.normal && approx_eq(self.d, other.d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plane_from_point_normal() {
        let point = Point3D::new(0.0, 0.0, 0.0);
        let normal = Vector3D::new(0.0, 0.0, 1.0);
        let plane = Plane::from_point_normal(&point, &normal).unwrap();
        assert!(plane.contains_point(&point));
    }

    #[test]
    fn test_plane_from_three_points() {
        let p1 = Point3D::new(0.0, 0.0, 0.0);
        let p2 = Point3D::new(1.0, 0.0, 0.0);
        let p3 = Point3D::new(0.0, 1.0, 0.0);
        let plane = Plane::from_three_points(&p1, &p2, &p3).unwrap();
        assert!(plane.contains_point(&p1));
        assert!(plane.contains_point(&p2));
        assert!(plane.contains_point(&p3));
    }

    #[test]
    fn test_distance_to_point() {
        let plane = Plane::from_point_normal(&Point3D::origin(), &Vector3D::unit_z()).unwrap();
        let point = Point3D::new(0.0, 0.0, 5.0);
        assert_eq!(plane.distance_to_point(&point), 5.0);
    }

    #[test]
    fn test_collinear_points() {
        let p1 = Point3D::new(0.0, 0.0, 0.0);
        let p2 = Point3D::new(1.0, 0.0, 0.0);
        let p3 = Point3D::new(2.0, 0.0, 0.0);
        assert!(Plane::from_three_points(&p1, &p2, &p3).is_err());
    }
}
