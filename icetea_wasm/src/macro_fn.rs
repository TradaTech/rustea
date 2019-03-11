#[macro_export]
macro_rules! require {
  ( $condition:expr, $message:expr ) => {
    {
      if !$condition {
        panic!($message);
      }
    }
  };
}

#[macro_export]
macro_rules! load {
  (u128,$key:expr) => {{ load($key).as_u128().unwrap() }};
  (u64,$key:expr) => {{ load($key).as_u64().unwrap() }};
  (f64,$key:expr) => {{ load($key).as_f64().unwrap() }};
  (u32,$key:expr) => {{ load($key).as_u32().unwrap() }};
  (bool,$key:expr) => {{ load($key).as_bool().unwrap() }};
  (String,$key:expr) => {{ load($key).as_string().unwrap() }};
}

#[macro_export]
macro_rules! save {
  ( u128, $key:expr, $value:expr ) => {{ save($key, &Value::from_u128($value)); }};
  ( u64, $key:expr, $value:expr ) => {{ save($key, &Value::from_u64($value)); }};
  ( f64, $key:expr, $value:expr ) => {{ save($key, &Value::from_f64($value)); }};
  ( u32, $key:expr, $value:expr ) => {{ save($key, &Value::from_u32($value)); }};
  ( String, $key:expr, $value:expr ) => {{ save($key, &Value::from_str(&$value)); }};
  ( bool, $key:expr, $value:expr ) => {{ save($key, &Value::from_bool($value)); }};
  ( $key:expr, $value:expr ) => {{ save($key, &$value.to_value()); }};
}

#[macro_export]
macro_rules! get_key {
  ( $( $x:expr ),* ) => {
    {
      let mut temp_vec = Vec::new();
      $(
        temp_vec.push($x);
      )*
      temp_vec.join("-")
    }
  };
}

#[macro_export]
macro_rules! array {
  ( $( $x:expr ),* ) => {
    {
      let temp_array = js_sys::Array::new();
      $(
        temp_array.push($x);
      )*
      temp_array
    }
  };
}