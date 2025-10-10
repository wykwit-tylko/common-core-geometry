import pytest
from common_core_geometry import Point3D, Vector3D, Ray, Sphere, Triangle, Plane, AABB, LineSegment


class TestRaySphereIntegration:
    def test_ray_sphere_intersection_workflow(self):
        sphere = Sphere(Point3D(0, 0, 0), 1.0)
        
        ray = Ray(Point3D(0, 0, 5), Vector3D(0, 0, -1).normalize())
        
        result = ray.intersect_sphere(sphere)
        assert result is not None
        
        t, point = result
        assert t > 0
        assert abs(point.z - 1.0) < 1e-6

    def test_ray_sphere_miss(self):
        sphere = Sphere(Point3D(0, 0, 0), 1.0)
        
        ray = Ray(Point3D(10, 10, 10), Vector3D(1, 1, 1).normalize())
        
        result = ray.intersect_sphere(sphere)
        if result is not None:
            t, _ = result
            assert t < 0

    def test_ray_from_inside_sphere(self):
        sphere = Sphere(Point3D(0, 0, 0), 2.0)
        
        ray = Ray(Point3D(0, 0, 0), Vector3D(1, 0, 0).normalize())
        
        result = ray.intersect_sphere(sphere)
        assert result is not None
        
        t, point = result
        assert abs(abs(point.x) - 2.0) < 1e-6
        assert abs(point.y) < 1e-6
        assert abs(point.z) < 1e-6


class TestRayPlaneIntegration:
    def test_ray_plane_intersection_perpendicular(self):
        plane = Plane.from_point_normal(Point3D(0, 0, 0), Vector3D(0, 0, 1).normalize())
        
        ray = Ray(Point3D(1, 1, 5), Vector3D(0, 0, -1).normalize())
        
        result = ray.intersect_plane(plane)
        assert result is not None
        
        assert abs(result.x - 1.0) < 1e-6
        assert abs(result.y - 1.0) < 1e-6
        assert abs(result.z) < 1e-6

    def test_ray_plane_intersection_oblique(self):
        plane = Plane.from_point_normal(Point3D(0, 0, 0), Vector3D(0, 1, 0).normalize())
        
        ray = Ray(Point3D(0, 5, 0), Vector3D(1, -1, 0).normalize())
        
        result = ray.intersect_plane(plane)
        assert result is not None
        
        assert abs(result.y) < 1e-6

    def test_ray_plane_parallel_miss(self):
        plane = Plane.from_point_normal(Point3D(0, 0, 0), Vector3D(0, 0, 1).normalize())
        
        ray = Ray(Point3D(0, 0, 5), Vector3D(1, 0, 0).normalize())
        
        result = ray.intersect_plane(plane)
        assert result is None


class TestRayTriangleIntegration:
    def test_ray_triangle_intersection_center_hit(self):
        triangle = Triangle(
            Point3D(-1, -1, 0),
            Point3D(1, -1, 0),
            Point3D(0, 1, 0)
        )
        
        ray = Ray(Point3D(0, 0, 5), Vector3D(0, 0, -1).normalize())
        
        result = ray.intersect_triangle(triangle)
        assert result is not None
        
        t, point = result
        assert t > 0
        assert abs(point.z) < 1e-6

    def test_ray_triangle_miss_outside(self):
        triangle = Triangle(
            Point3D(0, 0, 0),
            Point3D(1, 0, 0),
            Point3D(0, 1, 0)
        )
        
        ray = Ray(Point3D(10, 10, 5), Vector3D(0, 0, -1).normalize())
        
        result = ray.intersect_triangle(triangle)
        assert result is None

    def test_ray_triangle_edge_hit(self):
        triangle = Triangle(
            Point3D(0, 0, 0),
            Point3D(2, 0, 0),
            Point3D(0, 2, 0)
        )
        
        ray = Ray(Point3D(1, 0, 5), Vector3D(0, 0, -1).normalize())
        
        result = ray.intersect_triangle(triangle)
        assert result is not None


class TestTrianglePlaneConsistency:
    def test_triangle_plane_normal_matches(self):
        a = Point3D(0, 0, 0)
        b = Point3D(1, 0, 0)
        c = Point3D(0, 1, 0)
        
        triangle = Triangle(a, b, c)
        plane = Plane.from_three_points(a, b, c)
        
        tri_normal = triangle.normal()
        plane_normal = plane.normal
        
        assert abs(tri_normal.x - plane_normal.x) < 1e-6
        assert abs(tri_normal.y - plane_normal.y) < 1e-6
        assert abs(tri_normal.z - plane_normal.z) < 1e-6

    def test_triangle_vertices_on_plane(self):
        a = Point3D(1, 2, 3)
        b = Point3D(4, 5, 6)
        c = Point3D(7, 8, 10)
        
        triangle = Triangle(a, b, c)
        plane = Plane.from_three_points(a, b, c)
        
        assert plane.contains(a)
        assert plane.contains(b)
        assert plane.contains(c)

    def test_triangle_centroid_on_plane(self):
        a = Point3D(0, 0, 1)
        b = Point3D(3, 0, 1)
        c = Point3D(0, 3, 1)
        
        triangle = Triangle(a, b, c)
        plane = Plane.from_three_points(a, b, c)
        
        centroid = triangle.centroid()
        assert plane.contains(centroid)


class TestAABBContainment:
    def test_aabb_contains_all_vertices(self):
        points = [
            Point3D(0, 0, 0),
            Point3D(1, 0, 0),
            Point3D(0, 1, 0),
            Point3D(0, 0, 1),
            Point3D(1, 1, 1),
        ]
        
        aabb = AABB(Point3D(0, 0, 0), Point3D(1, 1, 1))
        
        for point in points:
            assert aabb.contains(point), f"AABB should contain {point}"

    def test_aabb_contains_center(self):
        aabb = AABB(Point3D(-5, -5, -5), Point3D(5, 5, 5))
        
        center = aabb.center()
        assert aabb.contains(center)
        
        assert abs(center.x) < 1e-6
        assert abs(center.y) < 1e-6
        assert abs(center.z) < 1e-6

    def test_nested_aabb_containment(self):
        outer = AABB(Point3D(-10, -10, -10), Point3D(10, 10, 10))
        inner = AABB(Point3D(-5, -5, -5), Point3D(5, 5, 5))
        
        assert outer.contains(inner.min)
        assert outer.contains(inner.max)
        assert outer.contains(inner.center())


class TestSphereAABBRelationship:
    def test_sphere_center_in_aabb(self):
        sphere = Sphere(Point3D(0, 0, 0), 1.0)
        aabb = AABB(Point3D(-2, -2, -2), Point3D(2, 2, 2))
        
        assert aabb.contains(sphere.center)

    def test_sphere_bounds_aabb(self):
        sphere = Sphere(Point3D(0, 0, 0), 1.0)
        
        aabb = AABB(Point3D(-1, -1, -1), Point3D(1, 1, 1))
        
        assert aabb.contains(sphere.center)
        
        assert aabb.contains(Point3D(1, 0, 0))
        assert aabb.contains(Point3D(-1, 0, 0))
        assert aabb.contains(Point3D(0, 1, 0))
        assert aabb.contains(Point3D(0, -1, 0))
        assert aabb.contains(Point3D(0, 0, 1))
        assert aabb.contains(Point3D(0, 0, -1))


class TestGeometricProperties:
    def test_line_segment_endpoints_distance(self):
        seg = LineSegment(Point3D(0, 0, 0), Point3D(3, 4, 0))
        
        length = seg.length()
        distance = seg.start.distance_to(seg.end)
        
        assert abs(length - distance) < 1e-6
        assert abs(length - 5.0) < 1e-6

    def test_sphere_volume_surface_area_ratio(self):
        sphere = Sphere(Point3D(0, 0, 0), 2.0)
        
        volume = sphere.volume()
        surface_area = sphere.surface_area()
        
        expected_volume = (4.0 / 3.0) * 3.141592653589793 * 8.0
        expected_surface_area = 4.0 * 3.141592653589793 * 4.0
        
        assert abs(volume - expected_volume) < 1e-6
        assert abs(surface_area - expected_surface_area) < 1e-6

    def test_triangle_area_by_vertices(self):
        triangle = Triangle(
            Point3D(0, 0, 0),
            Point3D(4, 0, 0),
            Point3D(0, 3, 0)
        )
        
        area = triangle.area()
        
        expected_area = 0.5 * 4.0 * 3.0
        assert abs(area - expected_area) < 1e-6


class TestVectorPointOperations:
    def test_ray_point_at_with_distance(self):
        origin = Point3D(0, 0, 0)
        direction = Vector3D(1, 0, 0).normalize()
        
        ray = Ray(origin, direction)
        
        point = ray.point_at(5.0)
        
        distance = origin.distance_to(point)
        assert abs(distance - 5.0) < 1e-6

    def test_midpoint_symmetry(self):
        p1 = Point3D(0, 0, 0)
        p2 = Point3D(10, 10, 10)
        
        midpoint_12 = p1.midpoint(p2)
        midpoint_21 = p2.midpoint(p1)
        
        assert abs(midpoint_12.x - midpoint_21.x) < 1e-6
        assert abs(midpoint_12.y - midpoint_21.y) < 1e-6
        assert abs(midpoint_12.z - midpoint_21.z) < 1e-6
        
        seg = LineSegment(p1, p2)
        seg_midpoint = seg.midpoint()
        
        assert abs(midpoint_12.x - seg_midpoint.x) < 1e-6
        assert abs(midpoint_12.y - seg_midpoint.y) < 1e-6
        assert abs(midpoint_12.z - seg_midpoint.z) < 1e-6

    def test_vector_normalization_preserves_direction(self):
        v = Vector3D(3, 4, 0)
        normalized = v.normalize()
        
        assert abs(normalized.magnitude() - 1.0) < 1e-6
        
        ratio = v.magnitude()
        assert abs(v.x / ratio - normalized.x) < 1e-6
        assert abs(v.y / ratio - normalized.y) < 1e-6


class TestComplexScenarios:
    def test_ray_through_triangle_centroid(self):
        triangle = Triangle(
            Point3D(0, 0, 0),
            Point3D(3, 0, 0),
            Point3D(0, 3, 0)
        )
        
        centroid = triangle.centroid()
        
        ray = Ray(Point3D(centroid.x, centroid.y, 10), Vector3D(0, 0, -1).normalize())
        
        result = ray.intersect_triangle(triangle)
        assert result is not None
        
        t, point = result
        assert abs(point.x - centroid.x) < 1e-4
        assert abs(point.y - centroid.y) < 1e-4

    def test_multiple_sphere_ray_intersections(self):
        spheres = [
            Sphere(Point3D(0, 0, 0), 1.0),
            Sphere(Point3D(0, 0, 5), 1.0),
            Sphere(Point3D(0, 0, 10), 1.0),
        ]
        
        ray = Ray(Point3D(0, 0, -5), Vector3D(0, 0, 1).normalize())
        
        hits = []
        for sphere in spheres:
            result = ray.intersect_sphere(sphere)
            if result:
                hits.append(result)
        
        assert len(hits) == 3
        
        hits.sort(key=lambda x: x[0])
        
        assert hits[0][0] < hits[1][0] < hits[2][0]

    def test_aabb_intersection_symmetric(self):
        aabb1 = AABB(Point3D(0, 0, 0), Point3D(2, 2, 2))
        aabb2 = AABB(Point3D(1, 1, 1), Point3D(3, 3, 3))
        
        assert aabb1.intersects(aabb2)
        assert aabb2.intersects(aabb1)
