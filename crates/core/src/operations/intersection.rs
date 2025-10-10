use crate::primitives::{Plane, Point3D, Ray, Sphere, Triangle, Vector3D, AABB};
use crate::utils::approx_zero;

#[inline]
pub fn ray_plane_intersection(ray: &Ray, plane: &Plane) -> Option<Point3D> {
    let denom = plane.normal.dot(&ray.direction);

    if approx_zero(denom) {
        return None;
    }

    let origin_as_vec = Vector3D::new(ray.origin.x, ray.origin.y, ray.origin.z);
    let t = -(plane.normal.dot(&origin_as_vec) + plane.d) / denom;

    if t < 0.0 {
        return None;
    }

    Some(ray.point_at(t))
}

#[inline]
pub fn ray_sphere_intersection(ray: &Ray, sphere: &Sphere) -> Option<(f64, f64)> {
    let oc = ray.origin - sphere.center;

    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * oc.dot(&ray.direction);
    let c = oc.dot(&oc) - sphere.radius * sphere.radius;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return None;
    }

    let sqrt_disc = discriminant.sqrt();
    let t1 = (-b - sqrt_disc) / (2.0 * a);
    let t2 = (-b + sqrt_disc) / (2.0 * a);

    Some((t1, t2))
}

#[inline]
pub fn ray_triangle_intersection(ray: &Ray, triangle: &Triangle) -> Option<f64> {
    let edge1 = triangle.b - triangle.a;
    let edge2 = triangle.c - triangle.a;

    let h = ray.direction.cross(&edge2);
    let a = edge1.dot(&h);

    if approx_zero(a) {
        return None;
    }

    let f = 1.0 / a;
    let s = ray.origin - triangle.a;
    let u = f * s.dot(&h);

    if !(0.0..=1.0).contains(&u) {
        return None;
    }

    let q = s.cross(&edge1);
    let v = f * ray.direction.dot(&q);

    if v < 0.0 || u + v > 1.0 {
        return None;
    }

    let t = f * edge2.dot(&q);

    if t > 0.0 {
        Some(t)
    } else {
        None
    }
}

#[inline]
pub fn ray_aabb_intersection(ray: &Ray, aabb: &AABB) -> Option<(f64, f64)> {
    let mut tmin = f64::NEG_INFINITY;
    let mut tmax = f64::INFINITY;

    for i in 0..3 {
        let origin_component = match i {
            0 => ray.origin.x,
            1 => ray.origin.y,
            _ => ray.origin.z,
        };

        let dir_component = match i {
            0 => ray.direction.x,
            1 => ray.direction.y,
            _ => ray.direction.z,
        };

        let min_component = match i {
            0 => aabb.min.x,
            1 => aabb.min.y,
            _ => aabb.min.z,
        };

        let max_component = match i {
            0 => aabb.max.x,
            1 => aabb.max.y,
            _ => aabb.max.z,
        };

        if approx_zero(dir_component) {
            if origin_component < min_component || origin_component > max_component {
                return None;
            }
        } else {
            let inv_d = 1.0 / dir_component;
            let mut t0 = (min_component - origin_component) * inv_d;
            let mut t1 = (max_component - origin_component) * inv_d;

            if t0 > t1 {
                std::mem::swap(&mut t0, &mut t1);
            }

            tmin = tmin.max(t0);
            tmax = tmax.min(t1);

            if tmin > tmax {
                return None;
            }
        }
    }

    if tmax < 0.0 {
        return None;
    }

    Some((tmin, tmax))
}

#[inline]
pub fn aabb_aabb_intersection(a: &AABB, b: &AABB) -> bool {
    a.intersects(b)
}

#[inline]
pub fn sphere_sphere_intersection(s1: &Sphere, s2: &Sphere) -> bool {
    let distance_squared = s1.center.distance_squared_to(&s2.center);
    let sum_radii = s1.radius + s2.radius;
    distance_squared <= sum_radii * sum_radii
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::{Point3D, Vector3D};
    use crate::utils::approx_eq;

    #[test]
    fn test_ray_plane_intersection() {
        let ray = Ray::new(Point3D::origin(), Vector3D::new(0.0, 0.0, 1.0)).unwrap();
        let plane =
            Plane::from_point_normal(&Point3D::new(0.0, 0.0, 5.0), &Vector3D::new(0.0, 0.0, 1.0))
                .unwrap();

        let intersection = ray_plane_intersection(&ray, &plane);
        assert!(intersection.is_some());

        let point = intersection.unwrap();
        assert!(approx_eq(point.z, 5.0));
    }

    #[test]
    fn test_ray_plane_parallel() {
        let ray = Ray::new(Point3D::origin(), Vector3D::new(1.0, 0.0, 0.0)).unwrap();
        let plane =
            Plane::from_point_normal(&Point3D::new(0.0, 0.0, 5.0), &Vector3D::new(0.0, 0.0, 1.0))
                .unwrap();

        let intersection = ray_plane_intersection(&ray, &plane);
        assert!(intersection.is_none());
    }

    #[test]
    fn test_ray_sphere_intersection_hit() {
        let ray = Ray::new(Point3D::origin(), Vector3D::new(0.0, 0.0, 1.0)).unwrap();
        let sphere = Sphere::new(Point3D::new(0.0, 0.0, 5.0), 1.0).unwrap();

        let intersection = ray_sphere_intersection(&ray, &sphere);
        assert!(intersection.is_some());

        let (t1, t2) = intersection.unwrap();
        assert!(approx_eq(t1, 4.0));
        assert!(approx_eq(t2, 6.0));
    }

    #[test]
    fn test_ray_sphere_intersection_miss() {
        let ray = Ray::new(Point3D::origin(), Vector3D::new(1.0, 0.0, 0.0)).unwrap();
        let sphere = Sphere::new(Point3D::new(0.0, 10.0, 0.0), 1.0).unwrap();

        let intersection = ray_sphere_intersection(&ray, &sphere);
        assert!(intersection.is_none());
    }

    #[test]
    fn test_ray_triangle_intersection_hit() {
        let ray = Ray::new(Point3D::new(0.5, 0.5, -1.0), Vector3D::new(0.0, 0.0, 1.0)).unwrap();
        let triangle = Triangle::new(
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
            Point3D::new(0.0, 1.0, 0.0),
        )
        .unwrap();

        let intersection = ray_triangle_intersection(&ray, &triangle);
        assert!(intersection.is_some());
        assert!(approx_eq(intersection.unwrap(), 1.0));
    }

    #[test]
    fn test_ray_triangle_intersection_miss() {
        let ray = Ray::new(Point3D::new(2.0, 2.0, -1.0), Vector3D::new(0.0, 0.0, 1.0)).unwrap();
        let triangle = Triangle::new(
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
            Point3D::new(0.0, 1.0, 0.0),
        )
        .unwrap();

        let intersection = ray_triangle_intersection(&ray, &triangle);
        assert!(intersection.is_none());
    }

    #[test]
    fn test_ray_aabb_intersection_hit() {
        let ray = Ray::new(Point3D::origin(), Vector3D::new(1.0, 0.0, 0.0)).unwrap();
        let aabb = AABB::new(Point3D::new(1.0, -1.0, -1.0), Point3D::new(3.0, 1.0, 1.0)).unwrap();

        let intersection = ray_aabb_intersection(&ray, &aabb);
        assert!(intersection.is_some());

        let (tmin, tmax) = intersection.unwrap();
        assert!(approx_eq(tmin, 1.0));
        assert!(approx_eq(tmax, 3.0));
    }

    #[test]
    fn test_ray_aabb_intersection_miss() {
        let ray = Ray::new(Point3D::origin(), Vector3D::new(1.0, 0.0, 0.0)).unwrap();
        let aabb = AABB::new(Point3D::new(1.0, 5.0, 5.0), Point3D::new(3.0, 6.0, 6.0)).unwrap();

        let intersection = ray_aabb_intersection(&ray, &aabb);
        assert!(intersection.is_none());
    }

    #[test]
    fn test_aabb_aabb_intersection_true() {
        let a = AABB::new(Point3D::new(0.0, 0.0, 0.0), Point3D::new(2.0, 2.0, 2.0)).unwrap();
        let b = AABB::new(Point3D::new(1.0, 1.0, 1.0), Point3D::new(3.0, 3.0, 3.0)).unwrap();

        assert!(aabb_aabb_intersection(&a, &b));
    }

    #[test]
    fn test_aabb_aabb_intersection_false() {
        let a = AABB::new(Point3D::new(0.0, 0.0, 0.0), Point3D::new(1.0, 1.0, 1.0)).unwrap();
        let b = AABB::new(Point3D::new(2.0, 2.0, 2.0), Point3D::new(3.0, 3.0, 3.0)).unwrap();

        assert!(!aabb_aabb_intersection(&a, &b));
    }

    #[test]
    fn test_sphere_sphere_intersection_true() {
        let s1 = Sphere::new(Point3D::new(0.0, 0.0, 0.0), 2.0).unwrap();
        let s2 = Sphere::new(Point3D::new(3.0, 0.0, 0.0), 2.0).unwrap();

        assert!(sphere_sphere_intersection(&s1, &s2));
    }

    #[test]
    fn test_sphere_sphere_intersection_false() {
        let s1 = Sphere::new(Point3D::new(0.0, 0.0, 0.0), 1.0).unwrap();
        let s2 = Sphere::new(Point3D::new(10.0, 0.0, 0.0), 1.0).unwrap();

        assert!(!sphere_sphere_intersection(&s1, &s2));
    }
}
