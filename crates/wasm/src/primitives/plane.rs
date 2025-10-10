use common_core_geometry::primitives as core;
use wasm_bindgen::prelude::*;

use crate::primitives::{Point3D, Vector3D};
use crate::utils::to_js_error;

#[wasm_bindgen]
pub struct Plane {
    pub(crate) inner: core::Plane,
}

#[wasm_bindgen]
impl Plane {
    #[wasm_bindgen(js_name = fromPointNormal)]
    pub fn from_point_normal(point: &Point3D, normal: &Vector3D) -> Result<Plane, JsValue> {
        let inner =
            core::Plane::from_point_normal(&point.inner, &normal.inner).map_err(to_js_error)?;
        Ok(Plane { inner })
    }

    #[wasm_bindgen(js_name = fromThreePoints)]
    pub fn from_three_points(p1: &Point3D, p2: &Point3D, p3: &Point3D) -> Result<Plane, JsValue> {
        let inner =
            core::Plane::from_three_points(&p1.inner, &p2.inner, &p3.inner).map_err(to_js_error)?;
        Ok(Plane { inner })
    }

    #[wasm_bindgen(getter)]
    pub fn normal(&self) -> Vector3D {
        Vector3D {
            inner: self.inner.normal,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn d(&self) -> f64 {
        self.inner.d
    }

    #[wasm_bindgen(js_name = distanceToPoint)]
    pub fn distance_to_point(&self, point: &Point3D) -> f64 {
        self.inner.distance_to_point(&point.inner)
    }

    #[wasm_bindgen(js_name = closestPoint)]
    pub fn closest_point(&self, point: &Point3D) -> Point3D {
        Point3D {
            inner: self.inner.closest_point(&point.inner),
        }
    }

    #[wasm_bindgen(js_name = containsPoint)]
    pub fn contains_point(&self, point: &Point3D) -> bool {
        self.inner.contains_point(&point.inner)
    }

    #[wasm_bindgen(js_name = flipNormal)]
    pub fn flip_normal(&self) -> Plane {
        Plane {
            inner: self.inner.flip_normal(),
        }
    }

    #[wasm_bindgen(js_name = isParallel)]
    pub fn is_parallel(&self, other: &Plane) -> bool {
        self.inner.is_parallel(&other.inner)
    }
}
