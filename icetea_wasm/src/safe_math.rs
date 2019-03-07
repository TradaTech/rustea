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
  fn mul(&self, b: u32) -> u32;
  fn div(&self, b: u32) -> u32;
  fn sub(&self, b: u32) -> u32;
  fn add(&self, b: u32) -> u32;
  fn modulo(&self, b: u32) -> u32;
}

impl SafeMath for u32 {
  fn mul(&self, b: u32) -> u32 {
    if *self == 0 || b == 0  {
      return 0;
    }
    let c = *self * b;
    require!(c / *self == b);
    return c;
  }

  fn div(&self, b: u32) -> u32 {
    require!(b > 0);
    let c = *self / b;
    return c;
  }

  fn sub(&self, b: u32) -> u32 {
    require!(b <= *self);
    let c = *self - b;
    return c;
  }

  fn add(&self, b: u32) -> u32 {
    let c = *self + b;
    require!(c >= *self);
    return c;
  }

  fn modulo(&self, b: u32) -> u32 {
    require!(b > 0);
    return *self % b;
  }
}