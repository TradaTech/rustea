macro_rules! require {
  ( $condition:expr ) => {
    {
      if !$condition {
        panic!("condition is not satisfied");
      }
    }
  };
}

pub trait SafeMath {
  fn mul(&self, b: u128) -> u128;
  fn div(&self, b: u128) -> u128;
  fn sub(&self, b: u128) -> u128;
  fn add(&self, b: u128) -> u128;
  fn modulo(&self, b: u128) -> u128;
}

impl SafeMath for u128 {
  fn mul(&self, b: u128) -> u128 {
    if *self == 0 || b == 0  {
      return 0;
    }
    let c = *self * b;
    require!(c / *self == b);
    return c;
  }

  fn div(&self, b: u128) -> u128 {
    require!(b > 0);
    let c = *self / b;
    return c;
  }

  fn sub(&self, b: u128) -> u128 {
    require!(b <= *self);
    let c = *self - b;
    return c;
  }

  fn add(&self, b: u128) -> u128 {
    let c = *self + b;
    require!(c >= *self);
    return c;
  }

  fn modulo(&self, b: u128) -> u128 {
    require!(b > 0);
    return *self % b;
  }
}