#![allow(dead_code)]
use js_sys::Object;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

pub fn change_to_js_value<T: for<'a> Deserialize<'a> + Serialize>(v: T) -> JsValue {
    serde_wasm_bindgen::to_value(&v).expect("change_to_js_value Err!")
}

pub fn change_to_object<T: for<'a> Deserialize<'a> + Serialize>(v: T) -> Object {
    let temp = serde_wasm_bindgen::to_value(&v).expect("change_to_js_value Err!");
    Object::from(temp)
}
