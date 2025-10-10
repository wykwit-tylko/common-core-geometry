use common_core_geometry::primitives as core;
use wasm_bindgen::prelude::*;

use crate::primitives::{Point3D, Vector3D};
use crate::utils::to_js_error;

#[wasm_bindgen]
pub struct Triangle {
    pub(crate) inner: core::Triangle,
}

#[wasm_bindgen]
impl Triangle {
    #[wasm_bindgen(constructor)]
    pub fn new(a: &Point3D, b: &Point3D, c: &Point3D) -> Result<Triangle, JsValue> {
        let inner = core::Triangle::new(a.inner, b.inner, c.inner).map_err(to_js_error)?;
        Ok(Triangle { inner })
    }

    #[wasm_bindgen(getter)]
    pub fn a(&self) -> Point3D {
        Point3D {
            inner: self.inner.a,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn b(&self) -> Point3D {
        Point3D {
            inner: self.inner.b,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn c(&self) -> Point3D {
        Point3D {
            inner: self.inner.c,
        }
    }

    pub fn normal(&self) -> Vector3D {
        Vector3D {
            inner: self.inner.normal(),
        }
    }

    pub fn area(&self) -> f64 {
        self.inner.area()
    }

    pub fn centroid(&self) -> Point3D {
        Point3D {
            inner: self.inner.centroid(),
        }
    }

    #[wasm_bindgen(js_name = barycentricCoords)]
    pub fn barycentric_coords(&self, point: &Point3D) -> Vec<f64> {
        let (u, v, w) = self.inner.barycentric_coords(&point.inner);
        vec![u, v, w]
    }

    #[wasm_bindgen(js_name = containsPoint)]
    pub fn contains_point(&self, point: &Point3D) -> bool {
        self.inner.contains_point(&point.inner)
    }
}
