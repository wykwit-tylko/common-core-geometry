use crate::primitives::Point3D;

#[inline]
pub fn manhattan_distance(p1: &Point3D, p2: &Point3D) -> f64 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs() + (p1.z - p2.z).abs()
}

#[inline]
pub fn chebyshev_distance(p1: &Point3D, p2: &Point3D) -> f64 {
    let dx = (p1.x - p2.x).abs();
    let dy = (p1.y - p2.y).abs();
    let dz = (p1.z - p2.z).abs();
    dx.max(dy).max(dz)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_distance() {
        let p1 = Point3D::new(0.0, 0.0, 0.0);
        let p2 = Point3D::new(3.0, 4.0, 5.0);
        assert_eq!(manhattan_distance(&p1, &p2), 12.0);
    }

    #[test]
    fn test_manhattan_distance_negative() {
        let p1 = Point3D::new(1.0, 2.0, 3.0);
        let p2 = Point3D::new(-1.0, -2.0, -3.0);
        assert_eq!(manhattan_distance(&p1, &p2), 12.0);
    }

    #[test]
    fn test_manhattan_distance_same_point() {
        let p = Point3D::new(1.0, 2.0, 3.0);
        assert_eq!(manhattan_distance(&p, &p), 0.0);
    }

    #[test]
    fn test_chebyshev_distance() {
        let p1 = Point3D::new(0.0, 0.0, 0.0);
        let p2 = Point3D::new(3.0, 4.0, 5.0);
        assert_eq!(chebyshev_distance(&p1, &p2), 5.0);
    }

    #[test]
    fn test_chebyshev_distance_negative() {
        let p1 = Point3D::new(1.0, 2.0, 3.0);
        let p2 = Point3D::new(-1.0, -2.0, -3.0);
        assert_eq!(chebyshev_distance(&p1, &p2), 6.0);
    }

    #[test]
    fn test_chebyshev_distance_same_point() {
        let p = Point3D::new(1.0, 2.0, 3.0);
        assert_eq!(chebyshev_distance(&p, &p), 0.0);
    }

    #[test]
    fn test_chebyshev_max_component() {
        let p1 = Point3D::new(0.0, 0.0, 0.0);
        let p2 = Point3D::new(1.0, 10.0, 2.0);
        assert_eq!(chebyshev_distance(&p1, &p2), 10.0);
    }
}
