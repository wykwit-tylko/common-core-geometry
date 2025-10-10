import pytest
from common_core_geometry import Vector3D, Point3D

def test_vector3d_creation():
    v = Vector3D(1.0, 2.0, 3.0)
    assert v.x == 1.0
    assert v.y == 2.0
    assert v.z == 3.0

def test_vector3d_unit_vectors():
    ux = Vector3D.unit_x()
    assert ux.x == 1.0 and ux.y == 0.0 and ux.z == 0.0
    
    uy = Vector3D.unit_y()
    assert uy.x == 0.0 and uy.y == 1.0 and uy.z == 0.0
    
    uz = Vector3D.unit_z()
    assert uz.x == 0.0 and uz.y == 0.0 and uz.z == 1.0

def test_vector3d_magnitude():
    v = Vector3D(3.0, 4.0, 0.0)
    assert abs(v.magnitude() - 5.0) < 1e-10

def test_vector3d_normalize():
    v = Vector3D(3.0, 4.0, 0.0)
    n = v.normalize()
    assert abs(n.magnitude() - 1.0) < 1e-10

def test_vector3d_dot_product():
    v1 = Vector3D(1.0, 0.0, 0.0)
    v2 = Vector3D(0.0, 1.0, 0.0)
    assert abs(v1.dot(v2)) < 1e-10

def test_vector3d_cross_product():
    v1 = Vector3D(1.0, 0.0, 0.0)
    v2 = Vector3D(0.0, 1.0, 0.0)
    cross = v1.cross(v2)
    assert abs(cross.x - 0.0) < 1e-10
    assert abs(cross.y - 0.0) < 1e-10
    assert abs(cross.z - 1.0) < 1e-10

def test_vector3d_add():
    v1 = Vector3D(1.0, 2.0, 3.0)
    v2 = Vector3D(4.0, 5.0, 6.0)
    v3 = v1 + v2
    assert abs(v3.x - 5.0) < 1e-10
    assert abs(v3.y - 7.0) < 1e-10
    assert abs(v3.z - 9.0) < 1e-10

def test_vector3d_sub():
    v1 = Vector3D(4.0, 5.0, 6.0)
    v2 = Vector3D(1.0, 2.0, 3.0)
    v3 = v1 - v2
    assert abs(v3.x - 3.0) < 1e-10
    assert abs(v3.y - 3.0) < 1e-10
    assert abs(v3.z - 3.0) < 1e-10

def test_vector3d_mul():
    v = Vector3D(1.0, 2.0, 3.0)
    v2 = v * 2.0
    assert abs(v2.x - 2.0) < 1e-10
    assert abs(v2.y - 4.0) < 1e-10
    assert abs(v2.z - 6.0) < 1e-10

def test_vector3d_parallel():
    v1 = Vector3D(1.0, 2.0, 3.0)
    v2 = Vector3D(2.0, 4.0, 6.0)
    assert v1.is_parallel(v2)

def test_vector3d_perpendicular():
    v1 = Vector3D(1.0, 0.0, 0.0)
    v2 = Vector3D(0.0, 1.0, 0.0)
    assert v1.is_perpendicular(v2)
