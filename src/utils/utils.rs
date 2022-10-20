#![allow(dead_code)]
use js_sys::Object;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

pub fn struct_to_js_value<T: for<'a> Deserialize<'a> + Serialize>(v: T) -> JsValue {
    serde_wasm_bindgen::to_value(&v).expect("struct_to_js_value Err!")
}

pub fn struct_to_object<T: for<'a> Deserialize<'a> + Serialize>(v: T) -> Object {
    let temp = serde_wasm_bindgen::to_value(&v).expect("struct_to_object Err!");
    Object::from(temp)
}

pub fn js_value_to_struct<T: for<'a> Deserialize<'a> + Serialize>(v: JsValue) -> T {
    serde_wasm_bindgen::from_value(v).expect("js_value_to_struct Err!")
}
