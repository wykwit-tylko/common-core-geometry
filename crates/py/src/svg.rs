use pyo3::prelude::*;
use common_core_geometry as core;

use crate::primitives::{PyPoint3D, PyVector3D, PySphere, PyTriangle, PyLineSegment, PyAABB};

#[pyclass(name = "Camera")]
#[derive(Clone)]
pub struct PyCamera {
    pub inner: core::svg::Camera,
}

#[pymethods]
impl PyCamera {
    #[staticmethod]
    #[pyo3(signature = (position, target, up, fov, aspect, near=0.1, far=100.0))]
    #[pyo3(text_signature = "(position, target, up, fov, aspect, near=0.1, far=100.0)")]
    /// Create a perspective camera.
    ///
    /// Args:
    ///     position (Point3D): Camera position in 3D space
    ///     target (Point3D): Point the camera is looking at
    ///     up (Vector3D): Up direction vector
    ///     fov (float): Field of view in degrees
    ///     aspect (float): Aspect ratio (width/height)
    ///     near (float): Near clipping plane distance (default: 0.1)
    ///     far (float): Far clipping plane distance (default: 100.0)
    ///
    /// Returns:
    ///     Camera: A perspective camera
    ///
    /// Example:
    ///     >>> from common_core_geometry import Point3D, Vector3D
    ///     >>> from common_core_geometry.svg import Camera
    ///     >>> camera = Camera.perspective(
    ///     ...     Point3D(4, 4, 4),
    ///     ...     Point3D(0, 0, 0),
    ///     ...     Vector3D(0, 1, 0),
    ///     ...     60.0,
    ///     ...     16/9
    ///     ... )
    pub fn perspective(
        position: &PyPoint3D,
        target: &PyPoint3D,
        up: &PyVector3D,
        fov: f64,
        aspect: f64,
        near: f64,
        far: f64,
    ) -> Self {
        PyCamera {
            inner: core::svg::Camera::perspective(
                position.inner,
                target.inner,
                up.inner,
                fov,
                aspect,
                near,
                far,
            ),
        }
    }

    #[staticmethod]
    #[pyo3(text_signature = "(position, target, up, width, height)")]
    /// Create an orthographic camera.
    ///
    /// Args:
    ///     position (Point3D): Camera position in 3D space
    ///     target (Point3D): Point the camera is looking at
    ///     up (Vector3D): Up direction vector
    ///     width (float): View width
    ///     height (float): View height
    ///
    /// Returns:
    ///     Camera: An orthographic camera
    ///
    /// Example:
    ///     >>> camera = Camera.orthographic(
    ///     ...     Point3D(10, 10, 10),
    ///     ...     Point3D(0, 0, 0),
    ///     ...     Vector3D(0, 1, 0),
    ///     ...     10.0,
    ///     ...     10.0
    ///     ... )
    pub fn orthographic(
        position: &PyPoint3D,
        target: &PyPoint3D,
        up: &PyVector3D,
        width: f64,
        height: f64,
    ) -> Self {
        PyCamera {
            inner: core::svg::Camera::orthographic(
                position.inner,
                target.inner,
                up.inner,
                width,
                height,
            ),
        }
    }

    fn __repr__(&self) -> String {
        match &self.inner {
            core::svg::Camera::Perspective { fov, aspect, .. } => {
                format!("Camera.perspective(fov={}, aspect={})", fov, aspect)
            }
            core::svg::Camera::Orthographic { width, height, .. } => {
                format!("Camera.orthographic(width={}, height={})", width, height)
            }
        }
    }
}

#[pyclass(name = "SVGRenderer")]
pub struct PySVGRenderer {
    inner: core::svg::SVGRenderer,
}

#[pymethods]
impl PySVGRenderer {
    #[new]
    #[pyo3(text_signature = "(width, height, camera, /)")]
    /// Create a new SVG renderer.
    ///
    /// Args:
    ///     width (int): Canvas width in pixels
    ///     height (int): Canvas height in pixels
    ///     camera (Camera): Camera for viewing the 3D scene
    ///
    /// Returns:
    ///     SVGRenderer: A new SVG renderer
    ///
    /// Example:
    ///     >>> from common_core_geometry import Point3D, Vector3D
    ///     >>> from common_core_geometry.svg import Camera, SVGRenderer
    ///     >>> camera = Camera.perspective(
    ///     ...     Point3D(4, 4, 4),
    ///     ...     Point3D(0, 0, 0),
    ///     ...     Vector3D(0, 1, 0),
    ///     ...     60.0,
    ///     ...     16/9
    ///     ... )
    ///     >>> renderer = SVGRenderer(800, 600, camera)
    pub fn new(width: usize, height: usize, camera: &PyCamera) -> Self {
        PySVGRenderer {
            inner: core::svg::SVGRenderer::new(width, height, camera.inner.clone()),
        }
    }

    #[pyo3(text_signature = "($self, color, /)")]
    /// Set the background color of the SVG.
    ///
    /// Args:
    ///     color (str): Color in CSS format (e.g., "#ffffff", "white")
    ///
    /// Example:
    ///     >>> renderer.set_background("#f0f0f0")
    pub fn set_background(&mut self, color: &str) {
        self.inner.set_background(color);
    }

    #[pyo3(signature = (point, color="#000000", size=3.0))]
    #[pyo3(text_signature = "($self, point, color='#000000', size=3.0)")]
    /// Add a point to the scene.
    ///
    /// Args:
    ///     point (Point3D): The point to render
    ///     color (str): Point color (default: "#000000")
    ///     size (float): Point radius in pixels (default: 3.0)
    ///
    /// Example:
    ///     >>> renderer.add_point(Point3D(0, 0, 0), color="#ff0000", size=5.0)
    pub fn add_point(&mut self, point: &PyPoint3D, color: &str, size: f64) {
        self.inner.add_point(&point.inner, color, size);
    }

    #[pyo3(signature = (segment, color="#000000", width=1.0))]
    #[pyo3(text_signature = "($self, segment, color='#000000', width=1.0)")]
    /// Add a line segment to the scene.
    ///
    /// Args:
    ///     segment (LineSegment): The line segment to render
    ///     color (str): Line color (default: "#000000")
    ///     width (float): Line width in pixels (default: 1.0)
    ///
    /// Example:
    ///     >>> from common_core_geometry import LineSegment
    ///     >>> segment = LineSegment(Point3D(0, 0, 0), Point3D(1, 1, 1))
    ///     >>> renderer.add_line_segment(segment, color="#00ff00", width=2.0)
    pub fn add_line_segment(&mut self, segment: &PyLineSegment, color: &str, width: f64) {
        self.inner.add_line_segment(&segment.inner, color, width);
    }

    #[pyo3(signature = (triangle, stroke="#000000", fill=None, width=1.0))]
    #[pyo3(text_signature = "($self, triangle, stroke='#000000', fill=None, width=1.0)")]
    /// Add a triangle to the scene.
    ///
    /// Args:
    ///     triangle (Triangle): The triangle to render
    ///     stroke (str): Stroke color (default: "#000000")
    ///     fill (str | None): Fill color or None for no fill (default: None)
    ///     width (float): Stroke width in pixels (default: 1.0)
    ///
    /// Example:
    ///     >>> from common_core_geometry import Triangle
    ///     >>> tri = Triangle(Point3D(0, 0, 0), Point3D(1, 0, 0), Point3D(0, 1, 0))
    ///     >>> renderer.add_triangle(tri, stroke="#0000ff", fill="#ccccff", width=1.5)
    pub fn add_triangle(
        &mut self,
        triangle: &PyTriangle,
        stroke: &str,
        fill: Option<&str>,
        width: f64,
    ) {
        self.inner.add_triangle(&triangle.inner, stroke, fill, width);
    }

    #[pyo3(signature = (sphere, color="#000000", width=1.0))]
    #[pyo3(text_signature = "($self, sphere, color='#000000', width=1.0)")]
    /// Add a sphere to the scene (rendered as wireframe).
    ///
    /// Args:
    ///     sphere (Sphere): The sphere to render
    ///     color (str): Wireframe color (default: "#000000")
    ///     width (float): Line width in pixels (default: 1.0)
    ///
    /// Example:
    ///     >>> from common_core_geometry import Sphere
    ///     >>> sphere = Sphere(Point3D(0, 0, 0), 1.0)
    ///     >>> renderer.add_sphere(sphere, color="#ff00ff", width=1.5)
    pub fn add_sphere(&mut self, sphere: &PySphere, color: &str, width: f64) {
        self.inner.add_sphere(&sphere.inner, color, width);
    }

    #[pyo3(signature = (aabb, color="#000000", width=1.0))]
    #[pyo3(text_signature = "($self, aabb, color='#000000', width=1.0)")]
    /// Add an axis-aligned bounding box to the scene.
    ///
    /// Args:
    ///     aabb (AABB): The bounding box to render
    ///     color (str): Line color (default: "#000000")
    ///     width (float): Line width in pixels (default: 1.0)
    ///
    /// Example:
    ///     >>> from common_core_geometry import AABB
    ///     >>> box = AABB(Point3D(0, 0, 0), Point3D(1, 1, 1))
    ///     >>> renderer.add_aabb(box, color="#00ffff", width=2.0)
    pub fn add_aabb(&mut self, aabb: &PyAABB, color: &str, width: f64) {
        self.inner.add_aabb(&aabb.inner, color, width);
    }

    #[pyo3(text_signature = "($self)")]
    /// Render the scene to an SVG string.
    ///
    /// Returns:
    ///     str: The SVG markup
    ///
    /// Example:
    ///     >>> svg = renderer.render()
    ///     >>> print(svg[:50])
    ///     <svg xmlns="http://www.w3.org/2000/svg" width="80
    pub fn render(&self) -> String {
        self.inner.to_svg_string()
    }

    #[pyo3(text_signature = "($self, path, /)")]
    /// Save the rendered SVG to a file.
    ///
    /// Args:
    ///     path (str): File path to save to
    ///
    /// Raises:
    ///     IOError: If the file cannot be written
    ///
    /// Example:
    ///     >>> renderer.save("scene.svg")
    pub fn save(&self, path: &str) -> PyResult<()> {
        self.inner
            .to_file(path)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(format!("Failed to save SVG: {}", e)))
    }

    fn __enter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __exit__(
        &mut self,
        _exc_type: Option<&Bound<'_, PyAny>>,
        _exc_val: Option<&Bound<'_, PyAny>>,
        _exc_tb: Option<&Bound<'_, PyAny>>,
    ) -> bool {
        false
    }

    fn __repr__(&self) -> String {
        format!("SVGRenderer()")
    }
}

pub fn register_svg_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    parent.add_class::<PyCamera>()?;
    parent.add_class::<PySVGRenderer>()?;
    Ok(())
}
