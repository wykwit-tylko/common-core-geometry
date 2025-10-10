use pyo3::prelude::*;
use common_core_geometry::primitives as core;

#[pyclass(name = "Point3D")]
#[derive(Clone)]
pub struct PyPoint3D {
    pub inner: core::Point3D,
}

#[pymethods]
impl PyPoint3D {
    #[new]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        PyPoint3D {
            inner: core::Point3D::new(x, y, z),
        }
    }

    #[staticmethod]
    pub fn origin() -> Self {
        PyPoint3D {
            inner: core::Point3D::origin(),
        }
    }

    #[getter]
    pub fn x(&self) -> f64 {
        self.inner.x
    }

    #[getter]
    pub fn y(&self) -> f64 {
        self.inner.y
    }

    #[getter]
    pub fn z(&self) -> f64 {
        self.inner.z
    }

    pub fn distance_to(&self, other: &PyPoint3D) -> f64 {
        self.inner.distance_to(&other.inner)
    }

    pub fn midpoint(&self, other: &PyPoint3D) -> PyPoint3D {
        PyPoint3D {
            inner: self.inner.midpoint(&other.inner),
        }
    }

    pub fn translate(&self, vector: &PyVector3D) -> PyPoint3D {
        PyPoint3D {
            inner: self.inner.translate(&vector.inner),
        }
    }

    fn __repr__(&self) -> String {
        format!("Point3D({}, {}, {})", self.inner.x, self.inner.y, self.inner.z)
    }

    fn __str__(&self) -> String {
        format!("({}, {}, {})", self.inner.x, self.inner.y, self.inner.z)
    }

    fn __getitem__(&self, idx: isize) -> PyResult<f64> {
        match idx {
            0 => Ok(self.inner.x),
            1 => Ok(self.inner.y),
            2 => Ok(self.inner.z),
            -3 => Ok(self.inner.x),
            -2 => Ok(self.inner.y),
            -1 => Ok(self.inner.z),
            _ => Err(pyo3::exceptions::PyIndexError::new_err("Index out of range")),
        }
    }

    fn __eq__(&self, other: &PyPoint3D) -> bool {
        self.inner == other.inner
    }

    fn __hash__(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        self.inner.x.to_bits().hash(&mut hasher);
        self.inner.y.to_bits().hash(&mut hasher);
        self.inner.z.to_bits().hash(&mut hasher);
        hasher.finish()
    }
}

#[pyclass(name = "Vector3D")]
#[derive(Clone)]
pub struct PyVector3D {
    pub inner: core::Vector3D,
}

#[pymethods]
impl PyVector3D {
    #[new]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        PyVector3D {
            inner: core::Vector3D::new(x, y, z),
        }
    }

    #[staticmethod]
    pub fn zero() -> Self {
        PyVector3D {
            inner: core::Vector3D::zero(),
        }
    }

    #[staticmethod]
    pub fn unit_x() -> Self {
        PyVector3D {
            inner: core::Vector3D::unit_x(),
        }
    }

    #[staticmethod]
    pub fn unit_y() -> Self {
        PyVector3D {
            inner: core::Vector3D::unit_y(),
        }
    }

    #[staticmethod]
    pub fn unit_z() -> Self {
        PyVector3D {
            inner: core::Vector3D::unit_z(),
        }
    }

    #[staticmethod]
    pub fn from_points(from: &PyPoint3D, to: &PyPoint3D) -> Self {
        PyVector3D {
            inner: core::Vector3D::from_points(&from.inner, &to.inner),
        }
    }

    #[getter]
    pub fn x(&self) -> f64 {
        self.inner.x
    }

    #[getter]
    pub fn y(&self) -> f64 {
        self.inner.y
    }

    #[getter]
    pub fn z(&self) -> f64 {
        self.inner.z
    }

    pub fn magnitude(&self) -> f64 {
        self.inner.magnitude()
    }

    pub fn normalize(&self) -> PyResult<PyVector3D> {
        match self.inner.normalize() {
            Ok(v) => Ok(PyVector3D { inner: v }),
            Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!("{}", e))),
        }
    }

    pub fn dot(&self, other: &PyVector3D) -> f64 {
        self.inner.dot(&other.inner)
    }

    pub fn cross(&self, other: &PyVector3D) -> PyVector3D {
        PyVector3D {
            inner: self.inner.cross(&other.inner),
        }
    }

    pub fn angle(&self, other: &PyVector3D) -> f64 {
        self.inner.angle(&other.inner)
    }

    pub fn project_onto(&self, other: &PyVector3D) -> PyVector3D {
        PyVector3D {
            inner: self.inner.project_onto(&other.inner),
        }
    }

    pub fn is_parallel(&self, other: &PyVector3D) -> bool {
        self.inner.is_parallel(&other.inner)
    }

    pub fn is_perpendicular(&self, other: &PyVector3D) -> bool {
        self.inner.is_perpendicular(&other.inner)
    }

    fn __repr__(&self) -> String {
        format!("Vector3D({}, {}, {})", self.inner.x, self.inner.y, self.inner.z)
    }

    fn __str__(&self) -> String {
        format!("<{}, {}, {}>", self.inner.x, self.inner.y, self.inner.z)
    }

    fn __add__(&self, other: &PyVector3D) -> PyVector3D {
        PyVector3D {
            inner: self.inner + other.inner,
        }
    }

    fn __sub__(&self, other: &PyVector3D) -> PyVector3D {
        PyVector3D {
            inner: self.inner - other.inner,
        }
    }

    fn __mul__(&self, scalar: f64) -> PyVector3D {
        PyVector3D {
            inner: self.inner * scalar,
        }
    }

    fn __rmul__(&self, scalar: f64) -> PyVector3D {
        PyVector3D {
            inner: self.inner * scalar,
        }
    }

    fn __truediv__(&self, scalar: f64) -> PyVector3D {
        PyVector3D {
            inner: self.inner / scalar,
        }
    }

    fn __neg__(&self) -> PyVector3D {
        PyVector3D {
            inner: -self.inner,
        }
    }

    fn __getitem__(&self, idx: isize) -> PyResult<f64> {
        match idx {
            0 => Ok(self.inner.x),
            1 => Ok(self.inner.y),
            2 => Ok(self.inner.z),
            -3 => Ok(self.inner.x),
            -2 => Ok(self.inner.y),
            -1 => Ok(self.inner.z),
            _ => Err(pyo3::exceptions::PyIndexError::new_err("Index out of range")),
        }
    }

    fn __eq__(&self, other: &PyVector3D) -> bool {
        self.inner == other.inner
    }

    fn __hash__(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        self.inner.x.to_bits().hash(&mut hasher);
        self.inner.y.to_bits().hash(&mut hasher);
        self.inner.z.to_bits().hash(&mut hasher);
        hasher.finish()
    }
}

#[pyclass(name = "Sphere")]
#[derive(Clone)]
pub struct PySphere {
    pub inner: core::Sphere,
}

#[pymethods]
impl PySphere {
    #[new]
    pub fn new(center: &PyPoint3D, radius: f64) -> PyResult<Self> {
        match core::Sphere::new(center.inner, radius) {
            Ok(s) => Ok(PySphere { inner: s }),
            Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!("{}", e))),
        }
    }

    #[getter]
    pub fn center(&self) -> PyPoint3D {
        PyPoint3D {
            inner: self.inner.center,
        }
    }

    #[getter]
    pub fn radius(&self) -> f64 {
        self.inner.radius
    }

    pub fn volume(&self) -> f64 {
        self.inner.volume()
    }

    pub fn surface_area(&self) -> f64 {
        self.inner.surface_area()
    }

    pub fn contains(&self, point: &PyPoint3D) -> bool {
        self.inner.contains_point(&point.inner)
    }

    fn __repr__(&self) -> String {
        format!("Sphere(center=Point3D({}, {}, {}), radius={})", 
                self.inner.center.x, self.inner.center.y, self.inner.center.z, self.inner.radius)
    }
}

#[pyclass(name = "Ray")]
#[derive(Clone)]
pub struct PyRay {
    pub inner: core::Ray,
}

#[pymethods]
impl PyRay {
    #[new]
    pub fn new(origin: &PyPoint3D, direction: &PyVector3D) -> PyResult<Self> {
        match core::Ray::new(origin.inner, direction.inner) {
            Ok(r) => Ok(PyRay { inner: r }),
            Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!("{}", e))),
        }
    }

    #[getter]
    pub fn origin(&self) -> PyPoint3D {
        PyPoint3D {
            inner: self.inner.origin,
        }
    }

    #[getter]
    pub fn direction(&self) -> PyVector3D {
        PyVector3D {
            inner: self.inner.direction,
        }
    }

    pub fn point_at(&self, t: f64) -> PyPoint3D {
        PyPoint3D {
            inner: self.inner.point_at(t),
        }
    }

    fn __repr__(&self) -> String {
        format!("Ray(origin=Point3D({}, {}, {}), direction=Vector3D({}, {}, {}))",
                self.inner.origin.x, self.inner.origin.y, self.inner.origin.z,
                self.inner.direction.x, self.inner.direction.y, self.inner.direction.z)
    }
}

#[pyclass(name = "Triangle")]
#[derive(Clone)]
pub struct PyTriangle {
    pub inner: core::Triangle,
}

#[pymethods]
impl PyTriangle {
    #[new]
    pub fn new(a: &PyPoint3D, b: &PyPoint3D, c: &PyPoint3D) -> PyResult<Self> {
        match core::Triangle::new(a.inner, b.inner, c.inner) {
            Ok(t) => Ok(PyTriangle { inner: t }),
            Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!("{}", e))),
        }
    }

    #[getter]
    pub fn a(&self) -> PyPoint3D {
        PyPoint3D { inner: self.inner.a }
    }

    #[getter]
    pub fn b(&self) -> PyPoint3D {
        PyPoint3D { inner: self.inner.b }
    }

    #[getter]
    pub fn c(&self) -> PyPoint3D {
        PyPoint3D { inner: self.inner.c }
    }

    pub fn area(&self) -> f64 {
        self.inner.area()
    }

    pub fn normal(&self) -> PyVector3D {
        PyVector3D {
            inner: self.inner.normal(),
        }
    }

    pub fn centroid(&self) -> PyPoint3D {
        PyPoint3D {
            inner: self.inner.centroid(),
        }
    }

    fn __repr__(&self) -> String {
        format!("Triangle(a=Point3D({}, {}, {}), b=Point3D({}, {}, {}), c=Point3D({}, {}, {}))",
                self.inner.a.x, self.inner.a.y, self.inner.a.z,
                self.inner.b.x, self.inner.b.y, self.inner.b.z,
                self.inner.c.x, self.inner.c.y, self.inner.c.z)
    }
}

#[pyclass(name = "Plane")]
#[derive(Clone)]
pub struct PyPlane {
    pub inner: core::Plane,
}

#[pymethods]
impl PyPlane {
    #[new]
    pub fn new() -> PyResult<Self> {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Cannot directly instantiate Plane. Use from_point_normal() or from_three_points()"
        ))
    }

    #[staticmethod]
    pub fn from_point_normal(point: &PyPoint3D, normal: &PyVector3D) -> PyResult<Self> {
        match core::Plane::from_point_normal(&point.inner, &normal.inner) {
            Ok(p) => Ok(PyPlane { inner: p }),
            Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!("{}", e))),
        }
    }

    #[staticmethod]
    pub fn from_three_points(a: &PyPoint3D, b: &PyPoint3D, c: &PyPoint3D) -> PyResult<Self> {
        match core::Plane::from_three_points(&a.inner, &b.inner, &c.inner) {
            Ok(p) => Ok(PyPlane { inner: p }),
            Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!("{}", e))),
        }
    }

    #[getter]
    pub fn normal(&self) -> PyVector3D {
        PyVector3D {
            inner: self.inner.normal,
        }
    }

    #[getter]
    pub fn d(&self) -> f64 {
        self.inner.d
    }

    pub fn distance_to(&self, point: &PyPoint3D) -> f64 {
        self.inner.distance_to_point(&point.inner)
    }

    pub fn contains(&self, point: &PyPoint3D) -> bool {
        self.inner.contains_point(&point.inner)
    }

    fn __repr__(&self) -> String {
        format!("Plane(normal=Vector3D({}, {}, {}), d={})",
                self.inner.normal.x, self.inner.normal.y, self.inner.normal.z, self.inner.d)
    }
}

#[pyclass(name = "AABB")]
#[derive(Clone)]
pub struct PyAABB {
    pub inner: core::AABB,
}

#[pymethods]
impl PyAABB {
    #[new]
    pub fn new(min: &PyPoint3D, max: &PyPoint3D) -> PyResult<Self> {
        match core::AABB::new(min.inner, max.inner) {
            Ok(a) => Ok(PyAABB { inner: a }),
            Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!("{}", e))),
        }
    }

    #[staticmethod]
    pub fn from_points(points: Vec<PyPoint3D>) -> PyResult<Self> {
        let core_points: Vec<core::Point3D> = points.iter().map(|p| p.inner).collect();
        match core::AABB::from_points(&core_points) {
            Ok(a) => Ok(PyAABB { inner: a }),
            Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!("{}", e))),
        }
    }

    #[getter]
    pub fn min(&self) -> PyPoint3D {
        PyPoint3D { inner: self.inner.min }
    }

    #[getter]
    pub fn max(&self) -> PyPoint3D {
        PyPoint3D { inner: self.inner.max }
    }

    pub fn center(&self) -> PyPoint3D {
        PyPoint3D {
            inner: self.inner.center(),
        }
    }

    pub fn volume(&self) -> f64 {
        self.inner.volume()
    }

    pub fn surface_area(&self) -> f64 {
        self.inner.surface_area()
    }

    pub fn contains(&self, point: &PyPoint3D) -> bool {
        self.inner.contains_point(&point.inner)
    }

    pub fn intersects(&self, other: &PyAABB) -> bool {
        self.inner.intersects(&other.inner)
    }

    fn __repr__(&self) -> String {
        format!("AABB(min=Point3D({}, {}, {}), max=Point3D({}, {}, {}))",
                self.inner.min.x, self.inner.min.y, self.inner.min.z,
                self.inner.max.x, self.inner.max.y, self.inner.max.z)
    }
}

#[pyclass(name = "LineSegment")]
#[derive(Clone)]
pub struct PyLineSegment {
    pub inner: core::LineSegment,
}

#[pymethods]
impl PyLineSegment {
    #[new]
    pub fn new(start: &PyPoint3D, end: &PyPoint3D) -> PyResult<Self> {
        match core::LineSegment::new(start.inner, end.inner) {
            Ok(l) => Ok(PyLineSegment { inner: l }),
            Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!("{}", e))),
        }
    }

    #[getter]
    pub fn start(&self) -> PyPoint3D {
        PyPoint3D {
            inner: self.inner.start,
        }
    }

    #[getter]
    pub fn end(&self) -> PyPoint3D {
        PyPoint3D {
            inner: self.inner.end,
        }
    }

    pub fn length(&self) -> f64 {
        self.inner.length()
    }

    pub fn midpoint(&self) -> PyPoint3D {
        PyPoint3D {
            inner: self.inner.midpoint(),
        }
    }

    pub fn point_at(&self, t: f64) -> PyPoint3D {
        PyPoint3D {
            inner: self.inner.point_at(t),
        }
    }

    fn __repr__(&self) -> String {
        format!("LineSegment(start=Point3D({}, {}, {}), end=Point3D({}, {}, {}))",
                self.inner.start.x, self.inner.start.y, self.inner.start.z,
                self.inner.end.x, self.inner.end.y, self.inner.end.z)
    }
}
