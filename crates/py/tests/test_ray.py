import pytest
from common_core_geometry import Point3D, Vector3D, Ray, Sphere, Plane, Triangle


class TestRayConstruction:
    def test_create_ray(self):
        origin = Point3D(1, 2, 3)
        direction = Vector3D(1, 0, 0)
        ray = Ray(origin, direction)
        
        assert ray.origin.x == 1.0
        assert ray.origin.y == 2.0
        assert ray.origin.z == 3.0
        assert abs(ray.direction.magnitude() - 1.0) < 1e-10

    def test_ray_normalizes_direction(self):
        origin = Point3D(0, 0, 0)
        direction = Vector3D(3, 4, 0)
        ray = Ray(origin, direction)
        
        assert abs(ray.direction.magnitude() - 1.0) < 1e-10
        assert abs(ray.direction.x - 0.6) < 1e-10
        assert abs(ray.direction.y - 0.8) < 1e-10

    def test_zero_direction_raises_error(self):
        origin = Point3D(0, 0, 0)
        direction = Vector3D(0, 0, 0)
        
        with pytest.raises(ValueError):
            Ray(origin, direction)


class TestRayPointAt:
    def test_point_at_zero(self):
        ray = Ray(Point3D(1, 2, 3), Vector3D(1, 0, 0))
        point = ray.point_at(0.0)
        
        assert abs(point.x - 1.0) < 1e-10
        assert abs(point.y - 2.0) < 1e-10
        assert abs(point.z - 3.0) < 1e-10

    def test_point_at_positive(self):
        ray = Ray(Point3D(0, 0, 0), Vector3D(1, 0, 0))
        point = ray.point_at(5.0)
        
        assert abs(point.x - 5.0) < 1e-10
        assert abs(point.y - 0.0) < 1e-10
        assert abs(point.z - 0.0) < 1e-10

    def test_point_at_negative(self):
        ray = Ray(Point3D(5, 0, 0), Vector3D(1, 0, 0))
        point = ray.point_at(-2.0)
        
        assert abs(point.x - 3.0) < 1e-10


class TestRaySphereIntersection:
    def test_ray_hits_sphere_front(self):
        sphere = Sphere(Point3D(0, 0, 0), 1.0)
        ray = Ray(Point3D(0, 0, 5), Vector3D(0, 0, -1))
        
        result = ray.intersect_sphere(sphere)
        assert result is not None
        
        t, point = result
        assert t > 0
        assert abs(t - 4.0) < 1e-10
        assert abs(point.z - 1.0) < 1e-10

    def test_ray_hits_sphere_center(self):
        sphere = Sphere(Point3D(0, 0, 5), 1.0)
        ray = Ray(Point3D(0, 0, 0), Vector3D(0, 0, 1))
        
        result = ray.intersect_sphere(sphere)
        assert result is not None
        
        t, point = result
        assert abs(t - 4.0) < 1e-10

    def test_ray_misses_sphere(self):
        sphere = Sphere(Point3D(0, 10, 0), 1.0)
        ray = Ray(Point3D(0, 0, 0), Vector3D(1, 0, 0))
        
        result = ray.intersect_sphere(sphere)
        assert result is None

    def test_ray_from_inside_sphere(self):
        sphere = Sphere(Point3D(0, 0, 0), 5.0)
        ray = Ray(Point3D(0, 0, 0), Vector3D(1, 0, 0))
        
        result = ray.intersect_sphere(sphere)
        assert result is not None

    def test_ray_tangent_to_sphere(self):
        sphere = Sphere(Point3D(0, 1, 0), 1.0)
        ray = Ray(Point3D(-5, 0, 0), Vector3D(1, 0, 0))
        
        result = ray.intersect_sphere(sphere)
        assert result is not None


class TestRayPlaneIntersection:
    def test_ray_hits_plane_perpendicular(self):
        plane = Plane.from_point_normal(Point3D(0, 0, 5), Vector3D(0, 0, 1))
        ray = Ray(Point3D(0, 0, 0), Vector3D(0, 0, 1))
        
        result = ray.intersect_plane(plane)
        assert result is not None
        
        point = result
        assert abs(point.z - 5.0) < 1e-10

    def test_ray_hits_plane_at_angle(self):
        plane = Plane.from_point_normal(Point3D(0, 0, 0), Vector3D(0, 1, 0))
        ray = Ray(Point3D(0, 5, 0), Vector3D(0, -1, 0))
        
        result = ray.intersect_plane(plane)
        assert result is not None
        
        point = result
        assert abs(point.y - 0.0) < 1e-10

    def test_ray_parallel_to_plane(self):
        plane = Plane.from_point_normal(Point3D(0, 0, 5), Vector3D(0, 0, 1))
        ray = Ray(Point3D(0, 0, 0), Vector3D(1, 0, 0))
        
        result = ray.intersect_plane(plane)
        assert result is None

    def test_ray_away_from_plane(self):
        plane = Plane.from_point_normal(Point3D(0, 0, -5), Vector3D(0, 0, 1))
        ray = Ray(Point3D(0, 0, 0), Vector3D(0, 0, 1))
        
        result = ray.intersect_plane(plane)
        assert result is None


class TestRayTriangleIntersection:
    def test_ray_hits_triangle_center(self):
        triangle = Triangle(
            Point3D(0, 0, 0),
            Point3D(1, 0, 0),
            Point3D(0, 1, 0)
        )
        ray = Ray(Point3D(0.25, 0.25, 5), Vector3D(0, 0, -1))
        
        result = ray.intersect_triangle(triangle)
        assert result is not None
        
        t, point = result
        assert t > 0
        assert abs(point.z - 0.0) < 1e-10

    def test_ray_hits_triangle_edge(self):
        triangle = Triangle(
            Point3D(0, 0, 0),
            Point3D(1, 0, 0),
            Point3D(0, 1, 0)
        )
        ray = Ray(Point3D(0.5, 0.0, 5), Vector3D(0, 0, -1))
        
        result = ray.intersect_triangle(triangle)
        assert result is not None

    def test_ray_hits_triangle_vertex(self):
        triangle = Triangle(
            Point3D(0, 0, 0),
            Point3D(1, 0, 0),
            Point3D(0, 1, 0)
        )
        ray = Ray(Point3D(0, 0, 5), Vector3D(0, 0, -1))
        
        result = ray.intersect_triangle(triangle)
        assert result is not None

    def test_ray_misses_triangle(self):
        triangle = Triangle(
            Point3D(0, 0, 0),
            Point3D(1, 0, 0),
            Point3D(0, 1, 0)
        )
        ray = Ray(Point3D(2, 2, 5), Vector3D(0, 0, -1))
        
        result = ray.intersect_triangle(triangle)
        assert result is None

    def test_ray_parallel_to_triangle(self):
        triangle = Triangle(
            Point3D(0, 0, 0),
            Point3D(1, 0, 0),
            Point3D(0, 1, 0)
        )
        ray = Ray(Point3D(0, 0, 5), Vector3D(1, 0, 0))
        
        result = ray.intersect_triangle(triangle)
        assert result is None

    def test_ray_away_from_triangle(self):
        triangle = Triangle(
            Point3D(0, 0, 0),
            Point3D(1, 0, 0),
            Point3D(0, 1, 0)
        )
        ray = Ray(Point3D(0.5, 0.5, -5), Vector3D(0, 0, -1))
        
        result = ray.intersect_triangle(triangle)
        assert result is None


class TestRayProtocols:
    def test_repr(self):
        ray = Ray(Point3D(1, 2, 3), Vector3D(1, 0, 0))
        r = repr(ray)
        
        assert "Ray" in r
        assert "Point3D" in r
        assert "Vector3D" in r
