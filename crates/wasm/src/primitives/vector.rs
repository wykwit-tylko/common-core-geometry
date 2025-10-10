use wasm_bindgen::prelude::*;
use common_core_geometry as core;
use crate::utils::to_js_error;

#[wasm_bindgen]
pub struct Vector3D {
    pub(crate) inner: core::Vector3D,
}

#[wasm_bindgen]
impl Vector3D {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, z: f64) -> Vector3D {
        Vector3D {
            inner: core::Vector3D::new(x, y, z),
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

    pub fn magnitude(&self) -> f64 {
        self.inner.magnitude()
    }

    pub fn normalize(&self) -> Result<Vector3D, JsValue> {
        self.inner
            .normalize()
            .map(|v| Vector3D { inner: v })
            .map_err(to_js_error)
    }

    pub fn dot(&self, other: &Vector3D) -> f64 {
        self.inner.dot(&other.inner)
    }

    pub fn cross(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            inner: self.inner.cross(&other.inner),
        }
    }

    pub fn add(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            inner: self.inner + other.inner,
        }
    }

    pub fn sub(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            inner: self.inner - other.inner,
        }
    }

    pub fn scale(&self, scalar: f64) -> Vector3D {
        Vector3D {
            inner: self.inner * scalar,
        }
    }

    pub fn angle(&self, other: &Vector3D) -> f64 {
        self.inner.angle(&other.inner)
    }

    #[wasm_bindgen(js_name = projectOnto)]
    pub fn project_onto(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            inner: self.inner.project_onto(&other.inner),
        }
    }

    #[wasm_bindgen(js_name = isParallel)]
    pub fn is_parallel(&self, other: &Vector3D) -> bool {
        self.inner.is_parallel(&other.inner)
    }

    #[wasm_bindgen(js_name = isPerpendicular)]
    pub fn is_perpendicular(&self, other: &Vector3D) -> bool {
        self.inner.is_perpendicular(&other.inner)
    }

    #[wasm_bindgen(js_name = toArray)]
    pub fn to_array(&self) -> Vec<f64> {
        vec![self.inner.x, self.inner.y, self.inner.z]
    }

    #[wasm_bindgen(js_name = fromArray)]
    pub fn from_array(arr: Vec<f64>) -> Result<Vector3D, JsValue> {
        if arr.len() != 3 {
            return Err(to_js_error("Array must have exactly 3 elements"));
        }
        Ok(Vector3D {
            inner: core::Vector3D::new(arr[0], arr[1], arr[2]),
        })
    }

    pub fn zero() -> Vector3D {
        Vector3D {
            inner: core::Vector3D::zero(),
        }
    }

    #[wasm_bindgen(js_name = unitX)]
    pub fn unit_x() -> Vector3D {
        Vector3D {
            inner: core::Vector3D::unit_x(),
        }
    }

    #[wasm_bindgen(js_name = unitY)]
    pub fn unit_y() -> Vector3D {
        Vector3D {
            inner: core::Vector3D::unit_y(),
        }
    }

    #[wasm_bindgen(js_name = unitZ)]
    pub fn unit_z() -> Vector3D {
        Vector3D {
            inner: core::Vector3D::unit_z(),
        }
    }

    #[wasm_bindgen(js_name = fromPoints)]
    pub fn from_points(from: &super::Point3D, to: &super::Point3D) -> Vector3D {
        Vector3D {
            inner: core::Vector3D::from_points(&from.inner, &to.inner),
        }
    }
}
