use common_core_geometry as core;
use wasm_bindgen::prelude::*;

use crate::utils::to_js_error;

#[wasm_bindgen]
pub struct Ray {
    pub(crate) inner: core::Ray,
}

#[wasm_bindgen]
impl Ray {
    #[wasm_bindgen(constructor)]
    pub fn new(origin: &super::Point3D, direction: &super::Vector3D) -> Result<Ray, JsValue> {
        core::Ray::new(origin.inner, direction.inner)
            .map(|r| Ray { inner: r })
            .map_err(to_js_error)
    }

    #[wasm_bindgen(getter)]
    pub fn origin(&self) -> super::Point3D {
        super::Point3D {
            inner: self.inner.origin,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn direction(&self) -> super::Vector3D {
        super::Vector3D {
            inner: self.inner.direction,
        }
    }

    #[wasm_bindgen(js_name = pointAt)]
    pub fn point_at(&self, t: f64) -> super::Point3D {
        super::Point3D {
            inner: self.inner.point_at(t),
        }
    }

    #[wasm_bindgen(js_name = intersectSphere)]
    pub fn intersect_sphere(&self, sphere: &super::Sphere) -> JsValue {
        match core::operations::ray_sphere_intersection(&self.inner, &sphere.inner) {
            Some((t1, t2)) => {
                let obj = js_sys::Object::new();
                js_sys::Reflect::set(&obj, &"t1".into(), &t1.into()).unwrap();
                js_sys::Reflect::set(&obj, &"t2".into(), &t2.into()).unwrap();
                obj.into()
            }
            None => JsValue::NULL,
        }
    }

    #[wasm_bindgen(js_name = intersectPlane)]
    pub fn intersect_plane(&self, plane: &super::Plane) -> JsValue {
        match core::operations::ray_plane_intersection(&self.inner, &plane.inner) {
            Some(point) => {
                let p = super::Point3D { inner: point };
                JsValue::from(p)
            }
            None => JsValue::NULL,
        }
    }

    #[wasm_bindgen(js_name = intersectTriangle)]
    pub fn intersect_triangle(&self, triangle: &super::Triangle) -> JsValue {
        match core::operations::ray_triangle_intersection(&self.inner, &triangle.inner) {
            Some(t) => {
                let obj = js_sys::Object::new();
                js_sys::Reflect::set(&obj, &"t".into(), &t.into()).unwrap();
                let point = self.inner.point_at(t);
                let p = super::Point3D { inner: point };
                js_sys::Reflect::set(&obj, &"point".into(), &JsValue::from(p)).unwrap();
                obj.into()
            }
            None => JsValue::NULL,
        }
    }
}
