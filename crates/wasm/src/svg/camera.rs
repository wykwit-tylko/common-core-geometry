use common_core_geometry::svg as core_svg;
use wasm_bindgen::prelude::*;

use crate::primitives::{Point3D, Vector3D};

#[wasm_bindgen]
pub struct Camera {
    pub(crate) inner: core_svg::camera::Camera,
}

#[wasm_bindgen]
impl Camera {
    #[wasm_bindgen(js_name = perspective)]
    pub fn perspective(
        position: &Point3D,
        target: &Point3D,
        up: &Vector3D,
        fov: f64,
        aspect: f64,
        near: f64,
        far: f64,
    ) -> Camera {
        Camera {
            inner: core_svg::camera::Camera::perspective(
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

    #[wasm_bindgen(js_name = orthographic)]
    pub fn orthographic(
        position: &Point3D,
        target: &Point3D,
        up: &Vector3D,
        width: f64,
        height: f64,
    ) -> Camera {
        Camera {
            inner: core_svg::camera::Camera::orthographic(
                position.inner,
                target.inner,
                up.inner,
                width,
                height,
            ),
        }
    }

    #[wasm_bindgen(js_name = viewMatrix)]
    pub fn view_matrix(&self) -> Vec<f64> {
        let matrix = self.inner.view_matrix();
        matrix.iter().flat_map(|row| row.iter().copied()).collect()
    }

    #[wasm_bindgen(js_name = projectionMatrix)]
    pub fn projection_matrix(&self) -> Vec<f64> {
        let matrix = self.inner.projection_matrix();
        matrix.iter().flat_map(|row| row.iter().copied()).collect()
    }
}
