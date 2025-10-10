import pytest
import math
from common_core_geometry import Point3D, LineSegment


class TestLineSegmentConstruction:
    def test_create_line_segment_valid(self):
        start = Point3D(0.0, 0.0, 0.0)
        end = Point3D(1.0, 0.0, 0.0)
        seg = LineSegment(start, end)
        assert seg.start == start
        assert seg.end == end

    def test_create_line_segment_3d(self):
        start = Point3D(1.0, 2.0, 3.0)
        end = Point3D(4.0, 5.0, 6.0)
        seg = LineSegment(start, end)
        assert seg.start == start
        assert seg.end == end

    def test_create_degenerate_segment_error(self):
        point = Point3D(1.0, 1.0, 1.0)
        with pytest.raises(ValueError):
            LineSegment(point, point)


class TestLineSegmentProperties:
    def test_line_segment_start_getter(self):
        start = Point3D(1.0, 2.0, 3.0)
        end = Point3D(4.0, 5.0, 6.0)
        seg = LineSegment(start, end)
        assert seg.start == start

    def test_line_segment_end_getter(self):
        start = Point3D(1.0, 2.0, 3.0)
        end = Point3D(4.0, 5.0, 6.0)
        seg = LineSegment(start, end)
        assert seg.end == end


class TestLineSegmentLength:
    def test_unit_segment_length(self):
        start = Point3D(0.0, 0.0, 0.0)
        end = Point3D(1.0, 0.0, 0.0)
        seg = LineSegment(start, end)
        assert math.isclose(seg.length(), 1.0, rel_tol=1e-9)

    def test_diagonal_segment_length(self):
        start = Point3D(0.0, 0.0, 0.0)
        end = Point3D(1.0, 1.0, 1.0)
        seg = LineSegment(start, end)
        assert math.isclose(seg.length(), math.sqrt(3), rel_tol=1e-9)

    def test_3_4_5_triangle_segment_length(self):
        start = Point3D(0.0, 0.0, 0.0)
        end = Point3D(3.0, 4.0, 0.0)
        seg = LineSegment(start, end)
        assert math.isclose(seg.length(), 5.0, rel_tol=1e-9)


class TestLineSegmentMidpoint:
    def test_unit_segment_midpoint(self):
        start = Point3D(0.0, 0.0, 0.0)
        end = Point3D(2.0, 0.0, 0.0)
        seg = LineSegment(start, end)
        mid = seg.midpoint()
        assert math.isclose(mid.x, 1.0, rel_tol=1e-9)
        assert math.isclose(mid.y, 0.0, abs_tol=1e-9)
        assert math.isclose(mid.z, 0.0, abs_tol=1e-9)

    def test_3d_segment_midpoint(self):
        start = Point3D(1.0, 2.0, 3.0)
        end = Point3D(5.0, 6.0, 7.0)
        seg = LineSegment(start, end)
        mid = seg.midpoint()
        assert math.isclose(mid.x, 3.0, rel_tol=1e-9)
        assert math.isclose(mid.y, 4.0, rel_tol=1e-9)
        assert math.isclose(mid.z, 5.0, rel_tol=1e-9)

    def test_negative_coords_midpoint(self):
        start = Point3D(-2.0, -4.0, -6.0)
        end = Point3D(2.0, 4.0, 6.0)
        seg = LineSegment(start, end)
        mid = seg.midpoint()
        assert math.isclose(mid.x, 0.0, abs_tol=1e-9)
        assert math.isclose(mid.y, 0.0, abs_tol=1e-9)
        assert math.isclose(mid.z, 0.0, abs_tol=1e-9)


class TestLineSegmentPointAt:
    def test_point_at_start(self):
        start = Point3D(0.0, 0.0, 0.0)
        end = Point3D(4.0, 0.0, 0.0)
        seg = LineSegment(start, end)
        point = seg.point_at(0.0)
        assert math.isclose(point.x, 0.0, abs_tol=1e-9)
        assert math.isclose(point.y, 0.0, abs_tol=1e-9)
        assert math.isclose(point.z, 0.0, abs_tol=1e-9)

    def test_point_at_end(self):
        start = Point3D(0.0, 0.0, 0.0)
        end = Point3D(4.0, 0.0, 0.0)
        seg = LineSegment(start, end)
        point = seg.point_at(1.0)
        assert math.isclose(point.x, 4.0, rel_tol=1e-9)
        assert math.isclose(point.y, 0.0, abs_tol=1e-9)
        assert math.isclose(point.z, 0.0, abs_tol=1e-9)

    def test_point_at_middle(self):
        start = Point3D(0.0, 0.0, 0.0)
        end = Point3D(4.0, 0.0, 0.0)
        seg = LineSegment(start, end)
        point = seg.point_at(0.5)
        assert math.isclose(point.x, 2.0, rel_tol=1e-9)
        assert math.isclose(point.y, 0.0, abs_tol=1e-9)
        assert math.isclose(point.z, 0.0, abs_tol=1e-9)

    def test_point_at_quarter(self):
        start = Point3D(0.0, 0.0, 0.0)
        end = Point3D(4.0, 0.0, 0.0)
        seg = LineSegment(start, end)
        point = seg.point_at(0.25)
        assert math.isclose(point.x, 1.0, rel_tol=1e-9)
        assert math.isclose(point.y, 0.0, abs_tol=1e-9)
        assert math.isclose(point.z, 0.0, abs_tol=1e-9)


class TestLineSegmentPythonProtocols:
    def test_repr(self):
        start = Point3D(1.0, 2.0, 3.0)
        end = Point3D(4.0, 5.0, 6.0)
        seg = LineSegment(start, end)
        repr_str = repr(seg)
        assert "LineSegment" in repr_str
        assert "1" in repr_str and "2" in repr_str and "3" in repr_str
        assert "4" in repr_str and "5" in repr_str and "6" in repr_str
