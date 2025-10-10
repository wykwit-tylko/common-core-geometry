use wasm_bindgen::JsValue;

pub fn to_js_error(err: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&format!("GeometryError: {}", err))
}
