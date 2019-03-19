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
  fn as_u64(&self) -> Option<u64>;
  fn as_u128(&self) -> Option<u128>;
  fn as_f64(&self) -> Option<f64>;

  fn from_u32(value: u32) -> Value;
  fn from_u64(value: u64) -> Value;
  fn from_u128(value: u128) -> Value;
  fn from_f64(value: f64) -> Value;
}

pub trait ToValue {
  fn to_value(&self) -> Value;
}

impl ToValue for u32 {
  fn to_value(&self) -> Value {
    Value::from_str(&self.to_string())
  }
}

impl ToValue for u64 {
  fn to_value(&self) -> Value {
    Value::from_str(&self.to_string())
  }
}

impl ToValue for u128 {
  fn to_value(&self) -> Value {
    Value::from_str(&self.to_string())
  }
}

impl ToValue for f64 {
  fn to_value(&self) -> Value {
    Value::from_str(&self.to_string())
  }
}

impl ToValue for bool {
  fn to_value(&self) -> Value {
    Value::from_bool(*self)
  }
}

impl ToValue for String {
  fn to_value(&self) -> Value {
    Value::from_str(&self)
  }
}

pub trait ToValueLifetime<'a> {
  fn to_value(&'a self) -> Value;
}

impl<'a> ToValueLifetime<'a> for &'a str {
  fn to_value(&'a self) -> Value {
    Value::from_str(self)
  }
}

impl Convert for Value {
  fn as_array(&self) -> Vec<Value> {
    let mut result = Vec::new();
    if self.is_null() || self.is_undefined() {
      return result;
    }
    let array = js_sys::Array::from(self);
    for x in 0..(array.length()) {
      match js_sys::Reflect::get(&array, &JsValue::from_f64(x as f64)) {
        Ok(x) => { result.push(x); }
        Err(_e) => {}
      }
    }
    return result;
  }

  fn as_u32(&self) -> Option<u32> {
    match self.as_string() {
      Some(value) => {
        let parsed = value.parse::<u32>();
        match parsed {
          Ok(x) => Some(x),
          Err(_e) => None
        }
      }
      None => None
    }
  }

  fn as_u64(&self) -> Option<u64> {
    match self.as_string() {
      Some(value) => {
        let parsed = value.parse::<u64>();
        match parsed {
          Ok(x) => Some(x),
          Err(_e) => None
        }
      }
      None => None
    }
  }

  fn as_u128(&self) -> Option<u128> {
    match self.as_string() {
      Some(value) => {
        let parsed = value.parse::<u128>();
        match parsed {
          Ok(x) => Some(x),
          Err(_e) => None
        }
      }
      None => None
    }
  }

  fn as_f64(&self) -> Option<f64> {
    match self.as_string() {
      Some(value) => {
        let parsed = value.parse::<f64>();
        match parsed {
          Ok(x) => Some(x),
          Err(_e) => None
        }
      }
      None => None
    }
  }

  fn from_u32(value: u32) -> Value {
    return Value::from_str(&value.to_string());
  }

  fn from_u64(value: u64) -> Value {
    return Value::from_str(&value.to_string());
  }

  fn from_u128(value: u128) -> Value {
    return Value::from_str(&value.to_string());
  }

  fn from_f64(value: f64) -> Value {
    return Value::from_str(&value.to_string());
  }
}