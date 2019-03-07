extern crate wasm_bindgen;
extern crate js_sys;
use wasm_bindgen::prelude::JsValue;

pub mod macro_fn;
pub mod safe_math;

pub type Value = JsValue;
pub type Array = js_sys::Array;

pub trait Convert {
  fn as_array(&self) -> Vec<Value>;
  fn as_u32(&self) -> Option<u32>;
  fn from_u32(value: u32) -> Value;
}

impl Convert for Value {
  fn as_array(&self) -> Vec<Value> {
    let mut result = Vec::new();
    if self.is_null() || self.is_undefined() {
      return result;
    }
    let array = js_sys::Array::from(self);
    for x in 0..(array.length()) {
      result.push(js_sys::Reflect::get(&array, &JsValue::from_f64(x as f64)).unwrap());
    }
    return result;
  }

  fn as_u32(&self) -> Option<u32> {
    let f64_value = self.as_f64();
    match f64_value {
      Some(x) => Some(x as u32),
      None => None
    }
  }

  fn from_u32(value: u32) -> Value {
    return Value::from_f64(value as f64);
  }
}