use wasm_bindgen::prelude::*;
use common_core_geometry as core;
use crate::utils::to_js_error;

#[wasm_bindgen]
pub struct Sphere {
    pub(crate) inner: core::Sphere,
}

#[wasm_bindgen]
impl Sphere {
    #[wasm_bindgen(constructor)]
    pub fn new(center: &super::Point3D, radius: f64) -> Result<Sphere, JsValue> {
        core::Sphere::new(center.inner, radius)
            .map(|s| Sphere { inner: s })
            .map_err(to_js_error)
    }

    #[wasm_bindgen(getter)]
    pub fn center(&self) -> super::Point3D {
        super::Point3D {
            inner: self.inner.center,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn radius(&self) -> f64 {
        self.inner.radius
    }

    pub fn volume(&self) -> f64 {
        self.inner.volume()
    }

    #[wasm_bindgen(js_name = surfaceArea)]
    pub fn surface_area(&self) -> f64 {
        self.inner.surface_area()
    }

    pub fn contains(&self, point: &super::Point3D) -> bool {
        self.inner.contains_point(&point.inner)
    }
}
