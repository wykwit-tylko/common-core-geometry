import pytest
import math
from common_core_geometry import Point3D, Plane, Vector3D


class TestPlaneConstruction:
    def test_create_plane_from_point_normal(self):
        point = Point3D(0.0, 0.0, 0.0)
        normal = Vector3D(0.0, 0.0, 1.0)
        plane = Plane.from_point_normal(point, normal)
        retrieved_normal = plane.normal
        assert math.isclose(retrieved_normal.x, 0.0, abs_tol=1e-9)
        assert math.isclose(retrieved_normal.y, 0.0, abs_tol=1e-9)
        assert math.isclose(retrieved_normal.z, 1.0, rel_tol=1e-9)

    def test_create_plane_from_three_points(self):
        a = Point3D(0.0, 0.0, 0.0)
        b = Point3D(1.0, 0.0, 0.0)
        c = Point3D(0.0, 1.0, 0.0)
        plane = Plane.from_three_points(a, b, c)
        normal = plane.normal
        assert math.isclose(abs(normal.z), 1.0, rel_tol=1e-9)

    def test_create_plane_zero_normal_error(self):
        point = Point3D(0.0, 0.0, 0.0)
        normal = Vector3D(0.0, 0.0, 0.0)
        with pytest.raises(ValueError):
            Plane.from_point_normal(point, normal)

    def test_create_plane_collinear_points_error(self):
        a = Point3D(0.0, 0.0, 0.0)
        b = Point3D(1.0, 0.0, 0.0)
        c = Point3D(2.0, 0.0, 0.0)
        with pytest.raises(ValueError):
            Plane.from_three_points(a, b, c)

    def test_cannot_call_constructor_directly(self):
        with pytest.raises(TypeError):
            Plane()


class TestPlaneProperties:
    def test_plane_normal_getter(self):
        point = Point3D(0.0, 0.0, 5.0)
        normal = Vector3D(0.0, 0.0, 1.0)
        plane = Plane.from_point_normal(point, normal)
        retrieved_normal = plane.normal
        assert math.isclose(retrieved_normal.magnitude(), 1.0, rel_tol=1e-9)

    def test_plane_d_getter(self):
        point = Point3D(0.0, 0.0, 5.0)
        normal = Vector3D(0.0, 0.0, 1.0)
        plane = Plane.from_point_normal(point, normal)
        d = plane.d
        assert isinstance(d, float)


class TestPlaneDistanceTo:
    def test_distance_to_point_on_plane(self):
        point = Point3D(0.0, 0.0, 0.0)
        normal = Vector3D(0.0, 0.0, 1.0)
        plane = Plane.from_point_normal(point, normal)
        test_point = Point3D(5.0, 5.0, 0.0)
        assert math.isclose(plane.distance_to(test_point), 0.0, abs_tol=1e-9)

    def test_distance_to_point_above_plane(self):
        point = Point3D(0.0, 0.0, 0.0)
        normal = Vector3D(0.0, 0.0, 1.0)
        plane = Plane.from_point_normal(point, normal)
        test_point = Point3D(0.0, 0.0, 5.0)
        assert math.isclose(abs(plane.distance_to(test_point)), 5.0, rel_tol=1e-9)

    def test_distance_to_point_below_plane(self):
        point = Point3D(0.0, 0.0, 0.0)
        normal = Vector3D(0.0, 0.0, 1.0)
        plane = Plane.from_point_normal(point, normal)
        test_point = Point3D(0.0, 0.0, -3.0)
        assert math.isclose(abs(plane.distance_to(test_point)), 3.0, rel_tol=1e-9)

    def test_distance_to_arbitrary_plane(self):
        point = Point3D(1.0, 1.0, 1.0)
        normal = Vector3D(1.0, 0.0, 0.0)
        plane = Plane.from_point_normal(point, normal)
        test_point = Point3D(5.0, 10.0, 10.0)
        assert math.isclose(abs(plane.distance_to(test_point)), 4.0, rel_tol=1e-9)


class TestPlaneContains:
    def test_contains_point_on_plane(self):
        point = Point3D(0.0, 0.0, 0.0)
        normal = Vector3D(0.0, 0.0, 1.0)
        plane = Plane.from_point_normal(point, normal)
        test_point = Point3D(100.0, -50.0, 0.0)
        assert plane.contains(test_point) is True

    def test_not_contains_point_off_plane(self):
        point = Point3D(0.0, 0.0, 0.0)
        normal = Vector3D(0.0, 0.0, 1.0)
        plane = Plane.from_point_normal(point, normal)
        test_point = Point3D(0.0, 0.0, 1.0)
        assert plane.contains(test_point) is False

    def test_contains_original_point(self):
        point = Point3D(5.0, 3.0, 2.0)
        normal = Vector3D(1.0, 1.0, 1.0)
        plane = Plane.from_point_normal(point, normal)
        assert plane.contains(point) is True

    def test_contains_three_points_used_in_construction(self):
        a = Point3D(0.0, 0.0, 0.0)
        b = Point3D(1.0, 0.0, 0.0)
        c = Point3D(0.0, 1.0, 0.0)
        plane = Plane.from_three_points(a, b, c)
        assert plane.contains(a) is True
        assert plane.contains(b) is True
        assert plane.contains(c) is True


class TestPlanePythonProtocols:
    def test_repr(self):
        point = Point3D(0.0, 0.0, 0.0)
        normal = Vector3D(0.0, 1.0, 0.0)
        plane = Plane.from_point_normal(point, normal)
        repr_str = repr(plane)
        assert "Plane" in repr_str
        assert "Vector3D" in repr_str
