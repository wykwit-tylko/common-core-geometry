use wasm_bindgen::prelude::*;
use common_core_geometry::primitives as core;
use crate::primitives::{Point3D, Vector3D};
use crate::utils::to_js_error;

#[wasm_bindgen]
pub struct AABB {
    pub(crate) inner: core::AABB,
}

#[wasm_bindgen]
impl AABB {
    #[wasm_bindgen(constructor)]
    pub fn new(min: &Point3D, max: &Point3D) -> Result<AABB, JsValue> {
        let inner = core::AABB::new(min.inner, max.inner)
            .map_err(to_js_error)?;
        Ok(AABB { inner })
    }

    #[wasm_bindgen(js_name = fromPoints)]
    pub fn from_points(points: Vec<Point3D>) -> Result<AABB, JsValue> {
        let core_points: Vec<core::Point3D> = points.iter().map(|p| p.inner).collect();
        let inner = core::AABB::from_points(&core_points)
            .map_err(to_js_error)?;
        Ok(AABB { inner })
    }

    #[wasm_bindgen(getter)]
    pub fn min(&self) -> Point3D {
        Point3D { inner: self.inner.min }
    }

    #[wasm_bindgen(getter)]
    pub fn max(&self) -> Point3D {
        Point3D { inner: self.inner.max }
    }

    pub fn center(&self) -> Point3D {
        Point3D { inner: self.inner.center() }
    }

    pub fn size(&self) -> Vector3D {
        Vector3D { inner: self.inner.size() }
    }

    pub fn volume(&self) -> f64 {
        self.inner.volume()
    }

    #[wasm_bindgen(js_name = surfaceArea)]
    pub fn surface_area(&self) -> f64 {
        self.inner.surface_area()
    }

    pub fn diagonal(&self) -> f64 {
        self.inner.diagonal()
    }

    #[wasm_bindgen(js_name = containsPoint)]
    pub fn contains_point(&self, point: &Point3D) -> bool {
        self.inner.contains_point(&point.inner)
    }

    pub fn intersects(&self, other: &AABB) -> bool {
        self.inner.intersects(&other.inner)
    }

    pub fn union(&self, other: &AABB) -> AABB {
        AABB { inner: self.inner.union(&other.inner) }
    }

    #[wasm_bindgen(js_name = expandByPoint)]
    pub fn expand_by_point(&self, point: &Point3D) -> AABB {
        AABB { inner: self.inner.expand_by_point(&point.inner) }
    }

    #[wasm_bindgen(js_name = expandByScalar)]
    pub fn expand_by_scalar(&self, amount: f64) -> AABB {
        AABB { inner: self.inner.expand_by_scalar(amount) }
    }
}
