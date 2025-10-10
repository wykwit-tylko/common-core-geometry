import pytest
from common_core_geometry import Point3D, Vector3D

def test_point3d_creation():
    p = Point3D(1.0, 2.0, 3.0)
    assert p.x == 1.0
    assert p.y == 2.0
    assert p.z == 3.0

def test_point3d_origin():
    p = Point3D.origin()
    assert p.x == 0.0
    assert p.y == 0.0
    assert p.z == 0.0

def test_point3d_distance():
    p1 = Point3D(0.0, 0.0, 0.0)
    p2 = Point3D(3.0, 4.0, 0.0)
    assert abs(p1.distance_to(p2) - 5.0) < 1e-10

def test_point3d_midpoint():
    p1 = Point3D(0.0, 0.0, 0.0)
    p2 = Point3D(2.0, 4.0, 6.0)
    mid = p1.midpoint(p2)
    assert abs(mid.x - 1.0) < 1e-10
    assert abs(mid.y - 2.0) < 1e-10
    assert abs(mid.z - 3.0) < 1e-10

def test_point3d_indexing():
    p = Point3D(1.0, 2.0, 3.0)
    assert p[0] == 1.0
    assert p[1] == 2.0
    assert p[2] == 3.0

def test_point3d_repr():
    p = Point3D(1.0, 2.0, 3.0)
    assert "Point3D" in repr(p)

def test_point3d_equality():
    p1 = Point3D(1.0, 2.0, 3.0)
    p2 = Point3D(1.0, 2.0, 3.0)
    assert p1 == p2
