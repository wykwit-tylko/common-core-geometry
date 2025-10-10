import pytest
import math
from common_core_geometry import Point3D, Triangle, Vector3D


class TestTriangleConstruction:
    def test_create_triangle_valid(self):
        a = Point3D(0.0, 0.0, 0.0)
        b = Point3D(1.0, 0.0, 0.0)
        c = Point3D(0.0, 1.0, 0.0)
        tri = Triangle(a, b, c)
        assert tri.a == a
        assert tri.b == b
        assert tri.c == c

    def test_create_triangle_3d(self):
        a = Point3D(0.0, 0.0, 0.0)
        b = Point3D(1.0, 0.0, 0.0)
        c = Point3D(0.0, 1.0, 1.0)
        tri = Triangle(a, b, c)
        assert tri.a == a
        assert tri.b == b
        assert tri.c == c

    def test_create_degenerate_triangle_error(self):
        a = Point3D(0.0, 0.0, 0.0)
        b = Point3D(1.0, 0.0, 0.0)
        c = Point3D(2.0, 0.0, 0.0)
        with pytest.raises(ValueError, match="collinear"):
            Triangle(a, b, c)


class TestTriangleProperties:
    def test_triangle_vertex_getters(self):
        a = Point3D(1.0, 0.0, 0.0)
        b = Point3D(0.0, 1.0, 0.0)
        c = Point3D(0.0, 0.0, 1.0)
        tri = Triangle(a, b, c)
        assert tri.a == a
        assert tri.b == b
        assert tri.c == c


class TestTriangleArea:
    def test_right_triangle_area(self):
        a = Point3D(0.0, 0.0, 0.0)
        b = Point3D(1.0, 0.0, 0.0)
        c = Point3D(0.0, 1.0, 0.0)
        tri = Triangle(a, b, c)
        assert math.isclose(tri.area(), 0.5, rel_tol=1e-9)

    def test_equilateral_triangle_area(self):
        a = Point3D(0.0, 0.0, 0.0)
        b = Point3D(1.0, 0.0, 0.0)
        c = Point3D(0.5, math.sqrt(3) / 2, 0.0)
        tri = Triangle(a, b, c)
        expected = math.sqrt(3) / 4
        assert math.isclose(tri.area(), expected, rel_tol=1e-9)

    def test_3d_triangle_area(self):
        a = Point3D(0.0, 0.0, 0.0)
        b = Point3D(1.0, 0.0, 0.0)
        c = Point3D(0.0, 1.0, 1.0)
        tri = Triangle(a, b, c)
        expected = math.sqrt(2) / 2
        assert math.isclose(tri.area(), expected, rel_tol=1e-9)


class TestTriangleNormal:
    def test_xy_plane_triangle_normal(self):
        a = Point3D(0.0, 0.0, 0.0)
        b = Point3D(1.0, 0.0, 0.0)
        c = Point3D(0.0, 1.0, 0.0)
        tri = Triangle(a, b, c)
        normal = tri.normal()
        assert math.isclose(normal.x, 0.0, abs_tol=1e-9)
        assert math.isclose(normal.y, 0.0, abs_tol=1e-9)
        assert math.isclose(abs(normal.z), 1.0, rel_tol=1e-9)

    def test_xz_plane_triangle_normal(self):
        a = Point3D(0.0, 0.0, 0.0)
        b = Point3D(1.0, 0.0, 0.0)
        c = Point3D(0.0, 0.0, 1.0)
        tri = Triangle(a, b, c)
        normal = tri.normal()
        assert math.isclose(normal.x, 0.0, abs_tol=1e-9)
        assert math.isclose(abs(normal.y), 1.0, rel_tol=1e-9)
        assert math.isclose(normal.z, 0.0, abs_tol=1e-9)

    def test_normal_is_unit_vector(self):
        a = Point3D(0.0, 0.0, 0.0)
        b = Point3D(2.0, 0.0, 0.0)
        c = Point3D(0.0, 3.0, 0.0)
        tri = Triangle(a, b, c)
        normal = tri.normal()
        assert math.isclose(normal.magnitude(), 1.0, rel_tol=1e-9)


class TestTriangleCentroid:
    def test_right_triangle_centroid(self):
        a = Point3D(0.0, 0.0, 0.0)
        b = Point3D(3.0, 0.0, 0.0)
        c = Point3D(0.0, 3.0, 0.0)
        tri = Triangle(a, b, c)
        centroid = tri.centroid()
        assert math.isclose(centroid.x, 1.0, rel_tol=1e-9)
        assert math.isclose(centroid.y, 1.0, rel_tol=1e-9)
        assert math.isclose(centroid.z, 0.0, abs_tol=1e-9)

    def test_3d_triangle_centroid(self):
        a = Point3D(0.0, 0.0, 0.0)
        b = Point3D(3.0, 0.0, 0.0)
        c = Point3D(0.0, 3.0, 3.0)
        tri = Triangle(a, b, c)
        centroid = tri.centroid()
        assert math.isclose(centroid.x, 1.0, rel_tol=1e-9)
        assert math.isclose(centroid.y, 1.0, rel_tol=1e-9)
        assert math.isclose(centroid.z, 1.0, rel_tol=1e-9)

    def test_unit_triangle_centroid(self):
        a = Point3D(0.0, 0.0, 0.0)
        b = Point3D(1.0, 0.0, 0.0)
        c = Point3D(0.0, 1.0, 0.0)
        tri = Triangle(a, b, c)
        centroid = tri.centroid()
        assert math.isclose(centroid.x, 1.0 / 3.0, rel_tol=1e-9)
        assert math.isclose(centroid.y, 1.0 / 3.0, rel_tol=1e-9)
        assert math.isclose(centroid.z, 0.0, abs_tol=1e-9)


class TestTrianglePythonProtocols:
    def test_repr(self):
        a = Point3D(0.0, 0.0, 0.0)
        b = Point3D(1.0, 0.0, 0.0)
        c = Point3D(0.0, 1.0, 1.0)
        tri = Triangle(a, b, c)
        repr_str = repr(tri)
        assert "Triangle" in repr_str
