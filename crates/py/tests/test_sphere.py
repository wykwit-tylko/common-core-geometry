import pytest
import math
from common_core_geometry import Point3D, Sphere


class TestSphereConstruction:
    def test_create_sphere_valid(self):
        center = Point3D(0.0, 0.0, 0.0)
        sphere = Sphere(center, 1.0)
        assert sphere.center == center
        assert sphere.radius == 1.0

    def test_create_sphere_non_origin(self):
        center = Point3D(5.0, -3.0, 2.0)
        sphere = Sphere(center, 2.5)
        assert sphere.center == center
        assert sphere.radius == 2.5

    def test_create_sphere_zero_radius_error(self):
        center = Point3D(0.0, 0.0, 0.0)
        with pytest.raises(ValueError, match="Sphere radius must be positive"):
            Sphere(center, 0.0)

    def test_create_sphere_negative_radius_error(self):
        center = Point3D(0.0, 0.0, 0.0)
        with pytest.raises(ValueError, match="Sphere radius must be positive"):
            Sphere(center, -1.0)


class TestSphereProperties:
    def test_sphere_center_getter(self):
        center = Point3D(1.0, 2.0, 3.0)
        sphere = Sphere(center, 1.0)
        retrieved_center = sphere.center
        assert retrieved_center.x == 1.0
        assert retrieved_center.y == 2.0
        assert retrieved_center.z == 3.0

    def test_sphere_radius_getter(self):
        sphere = Sphere(Point3D(0.0, 0.0, 0.0), 3.5)
        assert sphere.radius == 3.5


class TestSphereVolume:
    def test_unit_sphere_volume(self):
        sphere = Sphere(Point3D(0.0, 0.0, 0.0), 1.0)
        expected = (4.0 / 3.0) * math.pi
        assert math.isclose(sphere.volume(), expected, rel_tol=1e-9)

    def test_radius_2_sphere_volume(self):
        sphere = Sphere(Point3D(0.0, 0.0, 0.0), 2.0)
        expected = (4.0 / 3.0) * math.pi * 8.0
        assert math.isclose(sphere.volume(), expected, rel_tol=1e-9)

    def test_arbitrary_radius_sphere_volume(self):
        radius = 3.5
        sphere = Sphere(Point3D(1.0, 1.0, 1.0), radius)
        expected = (4.0 / 3.0) * math.pi * (radius ** 3)
        assert math.isclose(sphere.volume(), expected, rel_tol=1e-9)


class TestSphereSurfaceArea:
    def test_unit_sphere_surface_area(self):
        sphere = Sphere(Point3D(0.0, 0.0, 0.0), 1.0)
        expected = 4.0 * math.pi
        assert math.isclose(sphere.surface_area(), expected, rel_tol=1e-9)

    def test_radius_2_sphere_surface_area(self):
        sphere = Sphere(Point3D(0.0, 0.0, 0.0), 2.0)
        expected = 4.0 * math.pi * 4.0
        assert math.isclose(sphere.surface_area(), expected, rel_tol=1e-9)

    def test_arbitrary_radius_sphere_surface_area(self):
        radius = 5.0
        sphere = Sphere(Point3D(-1.0, 2.0, -3.0), radius)
        expected = 4.0 * math.pi * (radius ** 2)
        assert math.isclose(sphere.surface_area(), expected, rel_tol=1e-9)


class TestSphereContains:
    def test_contains_center_point(self):
        center = Point3D(0.0, 0.0, 0.0)
        sphere = Sphere(center, 1.0)
        assert sphere.contains(center) is True

    def test_contains_point_inside(self):
        sphere = Sphere(Point3D(0.0, 0.0, 0.0), 2.0)
        point = Point3D(0.5, 0.5, 0.5)
        assert sphere.contains(point) is True

    def test_contains_point_on_surface(self):
        sphere = Sphere(Point3D(0.0, 0.0, 0.0), 1.0)
        point = Point3D(1.0, 0.0, 0.0)
        assert sphere.contains(point) is True

    def test_not_contains_point_outside(self):
        sphere = Sphere(Point3D(0.0, 0.0, 0.0), 1.0)
        point = Point3D(2.0, 0.0, 0.0)
        assert sphere.contains(point) is False

    def test_contains_offset_sphere(self):
        center = Point3D(5.0, 5.0, 5.0)
        sphere = Sphere(center, 3.0)
        inside = Point3D(6.0, 5.0, 5.0)
        outside = Point3D(10.0, 5.0, 5.0)
        assert sphere.contains(inside) is True
        assert sphere.contains(outside) is False


class TestSpherePythonProtocols:
    def test_repr(self):
        sphere = Sphere(Point3D(1.0, 2.0, 3.0), 4.5)
        repr_str = repr(sphere)
        assert "Sphere" in repr_str
        assert "1" in repr_str
        assert "2" in repr_str
        assert "3" in repr_str
        assert "4.5" in repr_str
