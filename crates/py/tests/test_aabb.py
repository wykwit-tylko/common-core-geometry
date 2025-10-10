import pytest
import math
from common_core_geometry import Point3D, AABB


class TestAABBConstruction:
    def test_create_aabb_valid(self):
        min_point = Point3D(0.0, 0.0, 0.0)
        max_point = Point3D(1.0, 1.0, 1.0)
        aabb = AABB(min_point, max_point)
        assert aabb.min == min_point
        assert aabb.max == max_point

    def test_create_aabb_non_unit(self):
        min_point = Point3D(-5.0, -3.0, -2.0)
        max_point = Point3D(10.0, 7.0, 8.0)
        aabb = AABB(min_point, max_point)
        assert aabb.min == min_point
        assert aabb.max == max_point

    def test_create_aabb_invalid_bounds_error(self):
        min_point = Point3D(1.0, 1.0, 1.0)
        max_point = Point3D(0.0, 0.0, 0.0)
        with pytest.raises(ValueError):
            AABB(min_point, max_point)

    def test_create_aabb_from_points(self):
        points = [
            Point3D(0.0, 0.0, 0.0),
            Point3D(1.0, 2.0, 3.0),
            Point3D(-1.0, -2.0, 4.0),
        ]
        aabb = AABB.from_points(points)
        assert math.isclose(aabb.min.x, -1.0, rel_tol=1e-9)
        assert math.isclose(aabb.min.y, -2.0, rel_tol=1e-9)
        assert math.isclose(aabb.min.z, 0.0, abs_tol=1e-9)
        assert math.isclose(aabb.max.x, 1.0, rel_tol=1e-9)
        assert math.isclose(aabb.max.y, 2.0, rel_tol=1e-9)
        assert math.isclose(aabb.max.z, 4.0, rel_tol=1e-9)

    def test_create_aabb_from_single_point_error(self):
        points = [Point3D(5.0, 5.0, 5.0)]
        with pytest.raises(ValueError):
            AABB.from_points(points)

    def test_create_aabb_from_empty_points_error(self):
        with pytest.raises(ValueError):
            AABB.from_points([])


class TestAABBProperties:
    def test_aabb_min_getter(self):
        min_point = Point3D(1.0, 2.0, 3.0)
        max_point = Point3D(4.0, 5.0, 6.0)
        aabb = AABB(min_point, max_point)
        assert aabb.min == min_point

    def test_aabb_max_getter(self):
        min_point = Point3D(1.0, 2.0, 3.0)
        max_point = Point3D(4.0, 5.0, 6.0)
        aabb = AABB(min_point, max_point)
        assert aabb.max == max_point

    def test_aabb_center(self):
        min_point = Point3D(0.0, 0.0, 0.0)
        max_point = Point3D(2.0, 4.0, 6.0)
        aabb = AABB(min_point, max_point)
        center = aabb.center()
        assert math.isclose(center.x, 1.0, rel_tol=1e-9)
        assert math.isclose(center.y, 2.0, rel_tol=1e-9)
        assert math.isclose(center.z, 3.0, rel_tol=1e-9)


class TestAABBVolume:
    def test_unit_cube_volume(self):
        aabb = AABB(Point3D(0.0, 0.0, 0.0), Point3D(1.0, 1.0, 1.0))
        assert math.isclose(aabb.volume(), 1.0, rel_tol=1e-9)

    def test_2x3x4_box_volume(self):
        aabb = AABB(Point3D(0.0, 0.0, 0.0), Point3D(2.0, 3.0, 4.0))
        assert math.isclose(aabb.volume(), 24.0, rel_tol=1e-9)

    def test_negative_coords_volume(self):
        aabb = AABB(Point3D(-1.0, -1.0, -1.0), Point3D(1.0, 1.0, 1.0))
        assert math.isclose(aabb.volume(), 8.0, rel_tol=1e-9)


class TestAABBSurfaceArea:
    def test_unit_cube_surface_area(self):
        aabb = AABB(Point3D(0.0, 0.0, 0.0), Point3D(1.0, 1.0, 1.0))
        assert math.isclose(aabb.surface_area(), 6.0, rel_tol=1e-9)

    def test_2x3x4_box_surface_area(self):
        aabb = AABB(Point3D(0.0, 0.0, 0.0), Point3D(2.0, 3.0, 4.0))
        expected = 2 * (2*3 + 3*4 + 4*2)
        assert math.isclose(aabb.surface_area(), expected, rel_tol=1e-9)


class TestAABBContains:
    def test_contains_point_inside(self):
        aabb = AABB(Point3D(0.0, 0.0, 0.0), Point3D(2.0, 2.0, 2.0))
        point = Point3D(1.0, 1.0, 1.0)
        assert aabb.contains(point) is True

    def test_contains_point_on_min_corner(self):
        aabb = AABB(Point3D(0.0, 0.0, 0.0), Point3D(2.0, 2.0, 2.0))
        point = Point3D(0.0, 0.0, 0.0)
        assert aabb.contains(point) is True

    def test_contains_point_on_max_corner(self):
        aabb = AABB(Point3D(0.0, 0.0, 0.0), Point3D(2.0, 2.0, 2.0))
        point = Point3D(2.0, 2.0, 2.0)
        assert aabb.contains(point) is True

    def test_not_contains_point_outside(self):
        aabb = AABB(Point3D(0.0, 0.0, 0.0), Point3D(2.0, 2.0, 2.0))
        point = Point3D(3.0, 1.0, 1.0)
        assert aabb.contains(point) is False

    def test_contains_center(self):
        aabb = AABB(Point3D(-1.0, -1.0, -1.0), Point3D(1.0, 1.0, 1.0))
        center = aabb.center()
        assert aabb.contains(center) is True


class TestAABBIntersects:
    def test_intersects_overlapping_boxes(self):
        aabb1 = AABB(Point3D(0.0, 0.0, 0.0), Point3D(2.0, 2.0, 2.0))
        aabb2 = AABB(Point3D(1.0, 1.0, 1.0), Point3D(3.0, 3.0, 3.0))
        assert aabb1.intersects(aabb2) is True
        assert aabb2.intersects(aabb1) is True

    def test_intersects_touching_boxes(self):
        aabb1 = AABB(Point3D(0.0, 0.0, 0.0), Point3D(1.0, 1.0, 1.0))
        aabb2 = AABB(Point3D(1.0, 0.0, 0.0), Point3D(2.0, 1.0, 1.0))
        assert aabb1.intersects(aabb2) is True

    def test_not_intersects_separated_boxes(self):
        aabb1 = AABB(Point3D(0.0, 0.0, 0.0), Point3D(1.0, 1.0, 1.0))
        aabb2 = AABB(Point3D(2.0, 2.0, 2.0), Point3D(3.0, 3.0, 3.0))
        assert aabb1.intersects(aabb2) is False

    def test_intersects_one_inside_other(self):
        aabb1 = AABB(Point3D(0.0, 0.0, 0.0), Point3D(5.0, 5.0, 5.0))
        aabb2 = AABB(Point3D(1.0, 1.0, 1.0), Point3D(2.0, 2.0, 2.0))
        assert aabb1.intersects(aabb2) is True
        assert aabb2.intersects(aabb1) is True


class TestAABBPythonProtocols:
    def test_repr(self):
        min_point = Point3D(1.0, 2.0, 3.0)
        max_point = Point3D(4.0, 5.0, 6.0)
        aabb = AABB(min_point, max_point)
        repr_str = repr(aabb)
        assert "AABB" in repr_str
        assert "1" in repr_str and "2" in repr_str and "3" in repr_str
        assert "4" in repr_str and "5" in repr_str and "6" in repr_str
