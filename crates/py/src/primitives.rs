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
    #[pyo3(text_signature = "(x, y, z, /)")]
    /// Create a new 3D point.
    ///
    /// Args:
    ///     x (float): X coordinate
    ///     y (float): Y coordinate
    ///     z (float): Z coordinate
    ///
    /// Returns:
    ///     Point3D: A new point at the specified coordinates
    ///
    /// Example:
    ///     >>> from common_core_geometry import Point3D
    ///     >>> p = Point3D(1.0, 2.0, 3.0)
    ///     >>> p.x
    ///     1.0
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        PyPoint3D {
            inner: core::Point3D::new(x, y, z),
        }
    }

    #[staticmethod]
    #[pyo3(text_signature = "()")]
    /// Create a point at the origin (0, 0, 0).
    ///
    /// Returns:
    ///     Point3D: A point at the origin
    ///
    /// Example:
    ///     >>> from common_core_geometry import Point3D
    ///     >>> origin = Point3D.origin()
    ///     >>> origin.x, origin.y, origin.z
    ///     (0.0, 0.0, 0.0)
    pub fn origin() -> Self {
        PyPoint3D {
            inner: core::Point3D::origin(),
        }
    }

    #[getter]
    /// Get the X coordinate of the point.
    pub fn x(&self) -> f64 {
        self.inner.x
    }

    #[getter]
    /// Get the Y coordinate of the point.
    pub fn y(&self) -> f64 {
        self.inner.y
    }

    #[getter]
    /// Get the Z coordinate of the point.
    pub fn z(&self) -> f64 {
        self.inner.z
    }

    #[pyo3(text_signature = "($self, other, /)")]
    /// Calculate the Euclidean distance to another point.
    ///
    /// Args:
    ///     other (Point3D): The other point
    ///
    /// Returns:
    ///     float: The distance between the two points
    ///
    /// Example:
    ///     >>> p1 = Point3D(0, 0, 0)
    ///     >>> p2 = Point3D(3, 4, 0)
    ///     >>> p1.distance_to(p2)
    ///     5.0
    pub fn distance_to(&self, other: &PyPoint3D) -> f64 {
        self.inner.distance_to(&other.inner)
    }

    #[pyo3(text_signature = "($self, other, /)")]
    /// Calculate the midpoint between this point and another.
    ///
    /// Args:
    ///     other (Point3D): The other point
    ///
    /// Returns:
    ///     Point3D: The midpoint
    ///
    /// Example:
    ///     >>> p1 = Point3D(0, 0, 0)
    ///     >>> p2 = Point3D(2, 2, 2)
    ///     >>> mid = p1.midpoint(p2)
    ///     >>> (mid.x, mid.y, mid.z)
    ///     (1.0, 1.0, 1.0)
    pub fn midpoint(&self, other: &PyPoint3D) -> PyPoint3D {
        PyPoint3D {
            inner: self.inner.midpoint(&other.inner),
        }
    }

    #[pyo3(text_signature = "($self, vector, /)")]
    /// Translate this point by a vector.
    ///
    /// Args:
    ///     vector (Vector3D): The translation vector
    ///
    /// Returns:
    ///     Point3D: The translated point
    ///
    /// Example:
    ///     >>> p = Point3D(1, 2, 3)
    ///     >>> v = Vector3D(1, 0, 0)
    ///     >>> p2 = p.translate(v)
    ///     >>> (p2.x, p2.y, p2.z)
    ///     (2.0, 2.0, 3.0)
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
    #[pyo3(text_signature = "(x, y, z, /)")]
    /// Create a new 3D vector.
    ///
    /// Args:
    ///     x (float): X component
    ///     y (float): Y component
    ///     z (float): Z component
    ///
    /// Returns:
    ///     Vector3D: A new vector with the specified components
    ///
    /// Example:
    ///     >>> from common_core_geometry import Vector3D
    ///     >>> v = Vector3D(1.0, 2.0, 3.0)
    ///     >>> v.x
    ///     1.0
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        PyVector3D {
            inner: core::Vector3D::new(x, y, z),
        }
    }

    #[staticmethod]
    #[pyo3(text_signature = "()")]
    /// Create a zero vector (0, 0, 0).
    ///
    /// Returns:
    ///     Vector3D: A zero vector
    pub fn zero() -> Self {
        PyVector3D {
            inner: core::Vector3D::zero(),
        }
    }

    #[staticmethod]
    #[pyo3(text_signature = "()")]
    /// Create a unit vector along the X axis (1, 0, 0).
    ///
    /// Returns:
    ///     Vector3D: A unit X vector
    pub fn unit_x() -> Self {
        PyVector3D {
            inner: core::Vector3D::unit_x(),
        }
    }

    #[staticmethod]
    #[pyo3(text_signature = "()")]
    /// Create a unit vector along the Y axis (0, 1, 0).
    ///
    /// Returns:
    ///     Vector3D: A unit Y vector
    pub fn unit_y() -> Self {
        PyVector3D {
            inner: core::Vector3D::unit_y(),
        }
    }

    #[staticmethod]
    #[pyo3(text_signature = "()")]
    /// Create a unit vector along the Z axis (0, 0, 1).
    ///
    /// Returns:
    ///     Vector3D: A unit Z vector
    pub fn unit_z() -> Self {
        PyVector3D {
            inner: core::Vector3D::unit_z(),
        }
    }

    #[staticmethod]
    #[pyo3(text_signature = "(from, to, /)")]
    /// Create a vector from one point to another.
    ///
    /// Args:
    ///     from (Point3D): The starting point
    ///     to (Point3D): The ending point
    ///
    /// Returns:
    ///     Vector3D: The vector from 'from' to 'to'
    pub fn from_points(from: &PyPoint3D, to: &PyPoint3D) -> Self {
        PyVector3D {
            inner: core::Vector3D::from_points(&from.inner, &to.inner),
        }
    }

    #[getter]
    /// Get the X component of the vector.
    pub fn x(&self) -> f64 {
        self.inner.x
    }

    #[getter]
    /// Get the Y component of the vector.
    pub fn y(&self) -> f64 {
        self.inner.y
    }

    #[getter]
    /// Get the Z component of the vector.
    pub fn z(&self) -> f64 {
        self.inner.z
    }

    #[pyo3(text_signature = "($self)")]
    /// Calculate the magnitude (length) of the vector.
    ///
    /// Returns:
    ///     float: The magnitude of the vector
    ///
    /// Example:
    ///     >>> v = Vector3D(3, 4, 0)
    ///     >>> v.magnitude()
    ///     5.0
    pub fn magnitude(&self) -> f64 {
        self.inner.magnitude()
    }

    #[pyo3(text_signature = "($self)")]
    /// Normalize the vector to unit length.
    ///
    /// Returns:
    ///     Vector3D: The normalized vector
    ///
    /// Raises:
    ///     ValueError: If the vector is a zero vector
    ///
    /// Example:
    ///     >>> v = Vector3D(3, 0, 0)
    ///     >>> normalized = v.normalize()
    ///     >>> normalized.magnitude()
    ///     1.0
    pub fn normalize(&self) -> PyResult<PyVector3D> {
        match self.inner.normalize() {
            Ok(v) => Ok(PyVector3D { inner: v }),
            Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!("{}", e))),
        }
    }

    #[pyo3(text_signature = "($self, other, /)")]
    /// Calculate the dot product with another vector.
    ///
    /// Args:
    ///     other (Vector3D): The other vector
    ///
    /// Returns:
    ///     float: The dot product
    ///
    /// Example:
    ///     >>> v1 = Vector3D(1, 0, 0)
    ///     >>> v2 = Vector3D(0, 1, 0)
    ///     >>> v1.dot(v2)
    ///     0.0
    pub fn dot(&self, other: &PyVector3D) -> f64 {
        self.inner.dot(&other.inner)
    }

    #[pyo3(text_signature = "($self, other, /)")]
    /// Calculate the cross product with another vector.
    ///
    /// Args:
    ///     other (Vector3D): The other vector
    ///
    /// Returns:
    ///     Vector3D: The cross product vector
    ///
    /// Example:
    ///     >>> v1 = Vector3D(1, 0, 0)
    ///     >>> v2 = Vector3D(0, 1, 0)
    ///     >>> v3 = v1.cross(v2)
    ///     >>> (v3.x, v3.y, v3.z)
    ///     (0.0, 0.0, 1.0)
    pub fn cross(&self, other: &PyVector3D) -> PyVector3D {
        PyVector3D {
            inner: self.inner.cross(&other.inner),
        }
    }

    #[pyo3(text_signature = "($self, other, /)")]
    /// Calculate the angle in radians between this vector and another.
    ///
    /// Args:
    ///     other (Vector3D): The other vector
    ///
    /// Returns:
    ///     float: The angle in radians
    pub fn angle(&self, other: &PyVector3D) -> f64 {
        self.inner.angle(&other.inner)
    }

    #[pyo3(text_signature = "($self, other, /)")]
    /// Project this vector onto another vector.
    ///
    /// Args:
    ///     other (Vector3D): The vector to project onto
    ///
    /// Returns:
    ///     Vector3D: The projection
    pub fn project_onto(&self, other: &PyVector3D) -> PyVector3D {
        PyVector3D {
            inner: self.inner.project_onto(&other.inner),
        }
    }

    #[pyo3(text_signature = "($self, other, /)")]
    /// Check if this vector is parallel to another.
    ///
    /// Args:
    ///     other (Vector3D): The other vector
    ///
    /// Returns:
    ///     bool: True if parallel
    pub fn is_parallel(&self, other: &PyVector3D) -> bool {
        self.inner.is_parallel(&other.inner)
    }

    #[pyo3(text_signature = "($self, other, /)")]
    /// Check if this vector is perpendicular to another.
    ///
    /// Args:
    ///     other (Vector3D): The other vector
    ///
    /// Returns:
    ///     bool: True if perpendicular
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
    #[pyo3(text_signature = "(center, radius, /)")]
    /// Create a new sphere.
    ///
    /// Args:
    ///     center (Point3D): The center point of the sphere
    ///     radius (float): The radius (must be positive)
    ///
    /// Returns:
    ///     Sphere: A new sphere
    ///
    /// Raises:
    ///     ValueError: If radius is not positive
    ///
    /// Example:
    ///     >>> from common_core_geometry import Point3D, Sphere
    ///     >>> center = Point3D(0, 0, 0)
    ///     >>> sphere = Sphere(center, 5.0)
    ///     >>> sphere.radius
    ///     5.0
    pub fn new(center: &PyPoint3D, radius: f64) -> PyResult<Self> {
        match core::Sphere::new(center.inner, radius) {
            Ok(s) => Ok(PySphere { inner: s }),
            Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!("{}", e))),
        }
    }

    #[getter]
    /// Get the center point of the sphere.
    pub fn center(&self) -> PyPoint3D {
        PyPoint3D {
            inner: self.inner.center,
        }
    }

    #[getter]
    /// Get the radius of the sphere.
    pub fn radius(&self) -> f64 {
        self.inner.radius
    }

    #[pyo3(text_signature = "($self)")]
    /// Calculate the volume of the sphere.
    ///
    /// Returns:
    ///     float: The volume (4/3 * π * r³)
    ///
    /// Example:
    ///     >>> sphere = Sphere(Point3D(0, 0, 0), 1.0)
    ///     >>> sphere.volume()
    ///     4.1887902047863905
    pub fn volume(&self) -> f64 {
        self.inner.volume()
    }

    #[pyo3(text_signature = "($self)")]
    /// Calculate the surface area of the sphere.
    ///
    /// Returns:
    ///     float: The surface area (4 * π * r²)
    ///
    /// Example:
    ///     >>> sphere = Sphere(Point3D(0, 0, 0), 1.0)
    ///     >>> sphere.surface_area()
    ///     12.566370614359172
    pub fn surface_area(&self) -> f64 {
        self.inner.surface_area()
    }

    #[pyo3(text_signature = "($self, point, /)")]
    /// Check if a point is inside or on the sphere.
    ///
    /// Args:
    ///     point (Point3D): The point to check
    ///
    /// Returns:
    ///     bool: True if the point is inside or on the sphere
    ///
    /// Example:
    ///     >>> sphere = Sphere(Point3D(0, 0, 0), 5.0)
    ///     >>> sphere.contains(Point3D(1, 1, 1))
    ///     True
    ///     >>> sphere.contains(Point3D(10, 0, 0))
    ///     False
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
    #[pyo3(text_signature = "(origin, direction, /)")]
    /// Create a new ray.
    ///
    /// Args:
    ///     origin (Point3D): The starting point of the ray
    ///     direction (Vector3D): The direction vector (will be normalized)
    ///
    /// Returns:
    ///     Ray: A new ray
    ///
    /// Raises:
    ///     ValueError: If direction is a zero vector
    ///
    /// Example:
    ///     >>> from common_core_geometry import Point3D, Vector3D, Ray
    ///     >>> origin = Point3D(0, 0, 0)
    ///     >>> direction = Vector3D(1, 0, 0)
    ///     >>> ray = Ray(origin, direction)
    pub fn new(origin: &PyPoint3D, direction: &PyVector3D) -> PyResult<Self> {
        match core::Ray::new(origin.inner, direction.inner) {
            Ok(r) => Ok(PyRay { inner: r }),
            Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!("{}", e))),
        }
    }

    #[getter]
    /// Get the origin point of the ray.
    pub fn origin(&self) -> PyPoint3D {
        PyPoint3D {
            inner: self.inner.origin,
        }
    }

    #[getter]
    /// Get the direction vector of the ray (normalized).
    pub fn direction(&self) -> PyVector3D {
        PyVector3D {
            inner: self.inner.direction,
        }
    }

    #[pyo3(text_signature = "($self, t, /)")]
    /// Get a point along the ray at parameter t.
    ///
    /// Args:
    ///     t (float): The parameter value (origin + t * direction)
    ///
    /// Returns:
    ///     Point3D: The point at parameter t
    ///
    /// Example:
    ///     >>> ray = Ray(Point3D(0, 0, 0), Vector3D(1, 0, 0))
    ///     >>> point = ray.point_at(5.0)
    ///     >>> (point.x, point.y, point.z)
    ///     (5.0, 0.0, 0.0)
    pub fn point_at(&self, t: f64) -> PyPoint3D {
        PyPoint3D {
            inner: self.inner.point_at(t),
        }
    }

    #[pyo3(text_signature = "($self, sphere, /)")]
    /// Find the intersection with a sphere.
    ///
    /// Args:
    ///     sphere (Sphere): The sphere to test
    ///
    /// Returns:
    ///     Optional[Tuple[float, Point3D]]: The nearest intersection as (t, point), or None
    ///
    /// Example:
    ///     >>> ray = Ray(Point3D(0, 0, 0), Vector3D(1, 0, 0))
    ///     >>> sphere = Sphere(Point3D(5, 0, 0), 1.0)
    ///     >>> result = ray.intersect_sphere(sphere)
    ///     >>> if result:
    ///     ...     t, point = result
    pub fn intersect_sphere(&self, sphere: &PySphere) -> Option<(f64, PyPoint3D)> {
        use common_core_geometry::operations::ray_sphere_intersection;
        
        ray_sphere_intersection(&self.inner, &sphere.inner).map(|(t1, _t2)| {
            let point = self.inner.point_at(t1);
            (t1, PyPoint3D { inner: point })
        })
    }

    #[pyo3(text_signature = "($self, plane, /)")]
    /// Find the intersection with a plane.
    ///
    /// Args:
    ///     plane (Plane): The plane to test
    ///
    /// Returns:
    ///     Optional[Point3D]: The intersection point, or None if parallel
    ///
    /// Example:
    ///     >>> ray = Ray(Point3D(0, 0, 5), Vector3D(0, 0, -1))
    ///     >>> plane = Plane.from_point_normal(Point3D(0, 0, 0), Vector3D(0, 0, 1))
    ///     >>> point = ray.intersect_plane(plane)
    pub fn intersect_plane(&self, plane: &PyPlane) -> Option<PyPoint3D> {
        use common_core_geometry::operations::ray_plane_intersection;
        
        ray_plane_intersection(&self.inner, &plane.inner).map(|point| {
            PyPoint3D { inner: point }
        })
    }

    #[pyo3(text_signature = "($self, triangle, /)")]
    /// Find the intersection with a triangle.
    ///
    /// Args:
    ///     triangle (Triangle): The triangle to test
    ///
    /// Returns:
    ///     Optional[Tuple[float, Point3D]]: The intersection as (t, point), or None
    ///
    /// Example:
    ///     >>> ray = Ray(Point3D(0, 0, 5), Vector3D(0, 0, -1))
    ///     >>> tri = Triangle(Point3D(-1, -1, 0), Point3D(1, -1, 0), Point3D(0, 1, 0))
    ///     >>> result = ray.intersect_triangle(tri)
    pub fn intersect_triangle(&self, triangle: &PyTriangle) -> Option<(f64, PyPoint3D)> {
        use common_core_geometry::operations::ray_triangle_intersection;
        
        ray_triangle_intersection(&self.inner, &triangle.inner).map(|t| {
            let point = self.inner.point_at(t);
            (t, PyPoint3D { inner: point })
        })
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
    #[pyo3(text_signature = "(a, b, c, /)")]
    /// Create a new triangle from three points.
    ///
    /// Args:
    ///     a (Point3D): First vertex
    ///     b (Point3D): Second vertex
    ///     c (Point3D): Third vertex
    ///
    /// Returns:
    ///     Triangle: A new triangle
    ///
    /// Raises:
    ///     ValueError: If the three points are collinear
    ///
    /// Example:
    ///     >>> from common_core_geometry import Point3D, Triangle
    ///     >>> a = Point3D(0, 0, 0)
    ///     >>> b = Point3D(1, 0, 0)
    ///     >>> c = Point3D(0, 1, 0)
    ///     >>> tri = Triangle(a, b, c)
    pub fn new(a: &PyPoint3D, b: &PyPoint3D, c: &PyPoint3D) -> PyResult<Self> {
        match core::Triangle::new(a.inner, b.inner, c.inner) {
            Ok(t) => Ok(PyTriangle { inner: t }),
            Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!("{}", e))),
        }
    }

    #[getter]
    /// Get the first vertex of the triangle.
    pub fn a(&self) -> PyPoint3D {
        PyPoint3D { inner: self.inner.a }
    }

    #[getter]
    /// Get the second vertex of the triangle.
    pub fn b(&self) -> PyPoint3D {
        PyPoint3D { inner: self.inner.b }
    }

    #[getter]
    /// Get the third vertex of the triangle.
    pub fn c(&self) -> PyPoint3D {
        PyPoint3D { inner: self.inner.c }
    }

    #[pyo3(text_signature = "($self)")]
    /// Calculate the area of the triangle.
    ///
    /// Returns:
    ///     float: The area of the triangle
    ///
    /// Example:
    ///     >>> tri = Triangle(Point3D(0, 0, 0), Point3D(1, 0, 0), Point3D(0, 1, 0))
    ///     >>> tri.area()
    ///     0.5
    pub fn area(&self) -> f64 {
        self.inner.area()
    }

    #[pyo3(text_signature = "($self)")]
    /// Calculate the normal vector of the triangle.
    ///
    /// The normal is computed using the cross product of two edges
    /// and follows the right-hand rule based on vertex order.
    ///
    /// Returns:
    ///     Vector3D: The normalized normal vector
    ///
    /// Example:
    ///     >>> tri = Triangle(Point3D(0, 0, 0), Point3D(1, 0, 0), Point3D(0, 1, 0))
    ///     >>> normal = tri.normal()
    ///     >>> (normal.x, normal.y, normal.z)
    ///     (0.0, 0.0, 1.0)
    pub fn normal(&self) -> PyVector3D {
        PyVector3D {
            inner: self.inner.normal(),
        }
    }

    #[pyo3(text_signature = "($self)")]
    /// Calculate the centroid (center of mass) of the triangle.
    ///
    /// Returns:
    ///     Point3D: The centroid
    ///
    /// Example:
    ///     >>> tri = Triangle(Point3D(0, 0, 0), Point3D(3, 0, 0), Point3D(0, 3, 0))
    ///     >>> centroid = tri.centroid()
    ///     >>> (centroid.x, centroid.y, centroid.z)
    ///     (1.0, 1.0, 0.0)
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
    /// Cannot directly instantiate Plane. Use from_point_normal() or from_three_points().
    pub fn new() -> PyResult<Self> {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Cannot directly instantiate Plane. Use from_point_normal() or from_three_points()"
        ))
    }

    #[staticmethod]
    #[pyo3(text_signature = "(point, normal, /)")]
    /// Create a plane from a point and a normal vector.
    ///
    /// Args:
    ///     point (Point3D): A point on the plane
    ///     normal (Vector3D): The normal vector (will be normalized)
    ///
    /// Returns:
    ///     Plane: A new plane
    ///
    /// Raises:
    ///     ValueError: If normal is a zero vector
    ///
    /// Example:
    ///     >>> from common_core_geometry import Point3D, Vector3D, Plane
    ///     >>> point = Point3D(0, 0, 5)
    ///     >>> normal = Vector3D(0, 0, 1)
    ///     >>> plane = Plane.from_point_normal(point, normal)
    pub fn from_point_normal(point: &PyPoint3D, normal: &PyVector3D) -> PyResult<Self> {
        match core::Plane::from_point_normal(&point.inner, &normal.inner) {
            Ok(p) => Ok(PyPlane { inner: p }),
            Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!("{}", e))),
        }
    }

    #[staticmethod]
    #[pyo3(text_signature = "(a, b, c, /)")]
    /// Create a plane from three points.
    ///
    /// Args:
    ///     a (Point3D): First point
    ///     b (Point3D): Second point
    ///     c (Point3D): Third point
    ///
    /// Returns:
    ///     Plane: A new plane
    ///
    /// Raises:
    ///     ValueError: If the three points are collinear
    ///
    /// Example:
    ///     >>> a = Point3D(0, 0, 0)
    ///     >>> b = Point3D(1, 0, 0)
    ///     >>> c = Point3D(0, 1, 0)
    ///     >>> plane = Plane.from_three_points(a, b, c)
    pub fn from_three_points(a: &PyPoint3D, b: &PyPoint3D, c: &PyPoint3D) -> PyResult<Self> {
        match core::Plane::from_three_points(&a.inner, &b.inner, &c.inner) {
            Ok(p) => Ok(PyPlane { inner: p }),
            Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!("{}", e))),
        }
    }

    #[getter]
    /// Get the normal vector of the plane.
    pub fn normal(&self) -> PyVector3D {
        PyVector3D {
            inner: self.inner.normal,
        }
    }

    #[getter]
    /// Get the d coefficient of the plane equation (ax + by + cz + d = 0).
    pub fn d(&self) -> f64 {
        self.inner.d
    }

    #[pyo3(text_signature = "($self, point, /)")]
    /// Calculate the signed distance from a point to the plane.
    ///
    /// Args:
    ///     point (Point3D): The point to measure from
    ///
    /// Returns:
    ///     float: The signed distance (positive if on the side of the normal)
    ///
    /// Example:
    ///     >>> plane = Plane.from_point_normal(Point3D(0, 0, 0), Vector3D(0, 0, 1))
    ///     >>> plane.distance_to(Point3D(0, 0, 5))
    ///     5.0
    pub fn distance_to(&self, point: &PyPoint3D) -> f64 {
        self.inner.distance_to_point(&point.inner)
    }

    #[pyo3(text_signature = "($self, point, /)")]
    /// Check if a point lies on the plane (within epsilon tolerance).
    ///
    /// Args:
    ///     point (Point3D): The point to check
    ///
    /// Returns:
    ///     bool: True if the point is on the plane
    ///
    /// Example:
    ///     >>> plane = Plane.from_point_normal(Point3D(0, 0, 0), Vector3D(0, 0, 1))
    ///     >>> plane.contains(Point3D(5, 5, 0))
    ///     True
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
    #[pyo3(text_signature = "(min, max, /)")]
    /// Create a new axis-aligned bounding box.
    ///
    /// Args:
    ///     min (Point3D): The minimum corner point
    ///     max (Point3D): The maximum corner point
    ///
    /// Returns:
    ///     AABB: A new axis-aligned bounding box
    ///
    /// Raises:
    ///     ValueError: If min coordinates are not less than max coordinates
    ///
    /// Example:
    ///     >>> from common_core_geometry import Point3D, AABB
    ///     >>> min_point = Point3D(0, 0, 0)
    ///     >>> max_point = Point3D(10, 10, 10)
    ///     >>> aabb = AABB(min_point, max_point)
    pub fn new(min: &PyPoint3D, max: &PyPoint3D) -> PyResult<Self> {
        match core::AABB::new(min.inner, max.inner) {
            Ok(a) => Ok(PyAABB { inner: a }),
            Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!("{}", e))),
        }
    }

    #[staticmethod]
    #[pyo3(text_signature = "(points, /)")]
    /// Create an AABB that contains all the given points.
    ///
    /// Args:
    ///     points (List[Point3D]): The points to enclose
    ///
    /// Returns:
    ///     AABB: The minimum bounding box
    ///
    /// Raises:
    ///     ValueError: If the points list is empty
    ///
    /// Example:
    ///     >>> points = [Point3D(0, 0, 0), Point3D(5, 3, 2), Point3D(-1, 4, 1)]
    ///     >>> aabb = AABB.from_points(points)
    pub fn from_points(points: Vec<PyPoint3D>) -> PyResult<Self> {
        let core_points: Vec<core::Point3D> = points.iter().map(|p| p.inner).collect();
        match core::AABB::from_points(&core_points) {
            Ok(a) => Ok(PyAABB { inner: a }),
            Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!("{}", e))),
        }
    }

    #[getter]
    /// Get the minimum corner point of the AABB.
    pub fn min(&self) -> PyPoint3D {
        PyPoint3D { inner: self.inner.min }
    }

    #[getter]
    /// Get the maximum corner point of the AABB.
    pub fn max(&self) -> PyPoint3D {
        PyPoint3D { inner: self.inner.max }
    }

    #[pyo3(text_signature = "($self)")]
    /// Calculate the center point of the AABB.
    ///
    /// Returns:
    ///     Point3D: The center point
    ///
    /// Example:
    ///     >>> aabb = AABB(Point3D(0, 0, 0), Point3D(10, 10, 10))
    ///     >>> center = aabb.center()
    ///     >>> (center.x, center.y, center.z)
    ///     (5.0, 5.0, 5.0)
    pub fn center(&self) -> PyPoint3D {
        PyPoint3D {
            inner: self.inner.center(),
        }
    }

    #[pyo3(text_signature = "($self)")]
    /// Calculate the volume of the AABB.
    ///
    /// Returns:
    ///     float: The volume
    ///
    /// Example:
    ///     >>> aabb = AABB(Point3D(0, 0, 0), Point3D(2, 3, 4))
    ///     >>> aabb.volume()
    ///     24.0
    pub fn volume(&self) -> f64 {
        self.inner.volume()
    }

    #[pyo3(text_signature = "($self)")]
    /// Calculate the surface area of the AABB.
    ///
    /// Returns:
    ///     float: The surface area
    ///
    /// Example:
    ///     >>> aabb = AABB(Point3D(0, 0, 0), Point3D(2, 2, 2))
    ///     >>> aabb.surface_area()
    ///     24.0
    pub fn surface_area(&self) -> f64 {
        self.inner.surface_area()
    }

    #[pyo3(text_signature = "($self, point, /)")]
    /// Check if a point is inside or on the AABB.
    ///
    /// Args:
    ///     point (Point3D): The point to check
    ///
    /// Returns:
    ///     bool: True if the point is inside or on the AABB
    ///
    /// Example:
    ///     >>> aabb = AABB(Point3D(0, 0, 0), Point3D(10, 10, 10))
    ///     >>> aabb.contains(Point3D(5, 5, 5))
    ///     True
    pub fn contains(&self, point: &PyPoint3D) -> bool {
        self.inner.contains_point(&point.inner)
    }

    #[pyo3(text_signature = "($self, other, /)")]
    /// Check if this AABB intersects with another AABB.
    ///
    /// Args:
    ///     other (AABB): The other AABB
    ///
    /// Returns:
    ///     bool: True if the AABBs intersect
    ///
    /// Example:
    ///     >>> aabb1 = AABB(Point3D(0, 0, 0), Point3D(5, 5, 5))
    ///     >>> aabb2 = AABB(Point3D(3, 3, 3), Point3D(8, 8, 8))
    ///     >>> aabb1.intersects(aabb2)
    ///     True
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
    #[pyo3(text_signature = "(start, end, /)")]
    /// Create a new line segment.
    ///
    /// Args:
    ///     start (Point3D): The starting point
    ///     end (Point3D): The ending point
    ///
    /// Returns:
    ///     LineSegment: A new line segment
    ///
    /// Raises:
    ///     ValueError: If start and end points are the same
    ///
    /// Example:
    ///     >>> from common_core_geometry import Point3D, LineSegment
    ///     >>> start = Point3D(0, 0, 0)
    ///     >>> end = Point3D(10, 0, 0)
    ///     >>> segment = LineSegment(start, end)
    pub fn new(start: &PyPoint3D, end: &PyPoint3D) -> PyResult<Self> {
        match core::LineSegment::new(start.inner, end.inner) {
            Ok(l) => Ok(PyLineSegment { inner: l }),
            Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!("{}", e))),
        }
    }

    #[getter]
    /// Get the starting point of the line segment.
    pub fn start(&self) -> PyPoint3D {
        PyPoint3D {
            inner: self.inner.start,
        }
    }

    #[getter]
    /// Get the ending point of the line segment.
    pub fn end(&self) -> PyPoint3D {
        PyPoint3D {
            inner: self.inner.end,
        }
    }

    #[pyo3(text_signature = "($self)")]
    /// Calculate the length of the line segment.
    ///
    /// Returns:
    ///     float: The length
    ///
    /// Example:
    ///     >>> segment = LineSegment(Point3D(0, 0, 0), Point3D(3, 4, 0))
    ///     >>> segment.length()
    ///     5.0
    pub fn length(&self) -> f64 {
        self.inner.length()
    }

    #[pyo3(text_signature = "($self)")]
    /// Calculate the midpoint of the line segment.
    ///
    /// Returns:
    ///     Point3D: The midpoint
    ///
    /// Example:
    ///     >>> segment = LineSegment(Point3D(0, 0, 0), Point3D(10, 0, 0))
    ///     >>> mid = segment.midpoint()
    ///     >>> (mid.x, mid.y, mid.z)
    ///     (5.0, 0.0, 0.0)
    pub fn midpoint(&self) -> PyPoint3D {
        PyPoint3D {
            inner: self.inner.midpoint(),
        }
    }

    #[pyo3(text_signature = "($self, t, /)")]
    /// Get a point along the line segment at parameter t.
    ///
    /// Args:
    ///     t (float): Parameter in range [0, 1] (0 = start, 1 = end)
    ///
    /// Returns:
    ///     Point3D: The interpolated point
    ///
    /// Example:
    ///     >>> segment = LineSegment(Point3D(0, 0, 0), Point3D(10, 0, 0))
    ///     >>> point = segment.point_at(0.5)
    ///     >>> (point.x, point.y, point.z)
    ///     (5.0, 0.0, 0.0)
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
