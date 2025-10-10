use common_core_geometry::primitives as core;
use wasm_bindgen::prelude::*;

use crate::primitives::{Point3D, Vector3D};
use crate::utils::to_js_error;

#[wasm_bindgen]
pub struct LineSegment {
    pub(crate) inner: core::LineSegment,
}

#[wasm_bindgen]
impl LineSegment {
    #[wasm_bindgen(constructor)]
    pub fn new(start: &Point3D, end: &Point3D) -> Result<LineSegment, JsValue> {
        let inner = core::LineSegment::new(start.inner, end.inner).map_err(to_js_error)?;
        Ok(LineSegment { inner })
    }

    #[wasm_bindgen(getter)]
    pub fn start(&self) -> Point3D {
        Point3D {
            inner: self.inner.start,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn end(&self) -> Point3D {
        Point3D {
            inner: self.inner.end,
        }
    }

    pub fn length(&self) -> f64 {
        self.inner.length()
    }

    pub fn direction(&self) -> Vector3D {
        Vector3D {
            inner: self.inner.direction(),
        }
    }

    pub fn midpoint(&self) -> Point3D {
        Point3D {
            inner: self.inner.midpoint(),
        }
    }

    #[wasm_bindgen(js_name = pointAt)]
    pub fn point_at(&self, t: f64) -> Point3D {
        Point3D {
            inner: self.inner.point_at(t),
        }
    }

    #[wasm_bindgen(js_name = closestPoint)]
    pub fn closest_point(&self, point: &Point3D) -> Point3D {
        Point3D {
            inner: self.inner.closest_point(&point.inner),
        }
    }

    #[wasm_bindgen(js_name = distanceToPoint)]
    pub fn distance_to_point(&self, point: &Point3D) -> f64 {
        self.inner.distance_to_point(&point.inner)
    }
}
