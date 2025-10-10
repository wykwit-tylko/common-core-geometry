use common_core_geometry::svg as core_svg;
use wasm_bindgen::prelude::*;

use crate::primitives::{LineSegment, Point3D, Sphere, Triangle, AABB};
use crate::svg::camera::Camera;

#[wasm_bindgen]
pub struct SVGRenderer {
    pub(crate) inner: core_svg::renderer::SVGRenderer,
}

#[wasm_bindgen]
impl SVGRenderer {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, camera: &Camera) -> SVGRenderer {
        SVGRenderer {
            inner: core_svg::renderer::SVGRenderer::new(
                width as usize,
                height as usize,
                camera.inner.clone(),
            ),
        }
    }

    #[wasm_bindgen(js_name = setBackground)]
    pub fn set_background(&mut self, color: &str) {
        self.inner.set_background(color);
    }

    #[wasm_bindgen(js_name = addPoint)]
    pub fn add_point(&mut self, point: &Point3D, color: &str, size: f64) {
        self.inner.add_point(&point.inner, color, size);
    }

    #[wasm_bindgen(js_name = addLineSegment)]
    pub fn add_line_segment(&mut self, segment: &LineSegment, color: &str, width: f64) {
        self.inner.add_line_segment(&segment.inner, color, width);
    }

    #[wasm_bindgen(js_name = addTriangle)]
    pub fn add_triangle(
        &mut self,
        triangle: &Triangle,
        stroke: &str,
        fill: Option<String>,
        width: f64,
    ) {
        self.inner
            .add_triangle(&triangle.inner, stroke, fill.as_deref(), width);
    }

    #[wasm_bindgen(js_name = addSphere)]
    pub fn add_sphere(&mut self, sphere: &Sphere, color: &str, width: f64) {
        self.inner.add_sphere(&sphere.inner, color, width);
    }

    #[wasm_bindgen(js_name = addAabb)]
    pub fn add_aabb(&mut self, aabb: &AABB, color: &str, width: f64) {
        self.inner.add_aabb(&aabb.inner, color, width);
    }

    #[wasm_bindgen(js_name = toSvgString)]
    pub fn to_svg_string(&self) -> String {
        self.inner.to_svg_string()
    }
}
