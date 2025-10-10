use wasm_bindgen::prelude::*;
use common_core_geometry as core;
use crate::utils::to_js_error;

#[wasm_bindgen]
pub struct Point3D {
    pub(crate) inner: core::Point3D,
}

#[wasm_bindgen]
impl Point3D {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, z: f64) -> Point3D {
        Point3D {
            inner: core::Point3D::new(x, y, z),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f64 {
        self.inner.x
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f64 {
        self.inner.y
    }

    #[wasm_bindgen(getter)]
    pub fn z(&self) -> f64 {
        self.inner.z
    }

    #[wasm_bindgen(js_name = distanceTo)]
    pub fn distance_to(&self, other: &Point3D) -> f64 {
        self.inner.distance_to(&other.inner)
    }

    pub fn midpoint(&self, other: &Point3D) -> Point3D {
        Point3D {
            inner: self.inner.midpoint(&other.inner),
        }
    }

    pub fn translate(&self, vector: &super::Vector3D) -> Point3D {
        Point3D {
            inner: self.inner.translate(&vector.inner),
        }
    }

    #[wasm_bindgen(js_name = toArray)]
    pub fn to_array(&self) -> Vec<f64> {
        vec![self.inner.x, self.inner.y, self.inner.z]
    }

    #[wasm_bindgen(js_name = fromArray)]
    pub fn from_array(arr: Vec<f64>) -> Result<Point3D, JsValue> {
        if arr.len() != 3 {
            return Err(to_js_error("Array must have exactly 3 elements"));
        }
        Ok(Point3D {
            inner: core::Point3D::new(arr[0], arr[1], arr[2]),
        })
    }

    pub fn origin() -> Point3D {
        Point3D {
            inner: core::Point3D::origin(),
        }
    }
}
