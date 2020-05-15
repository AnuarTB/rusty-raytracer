use std::ops::{Add, Sub, Mul};

const EPS: f64 = 1e-12;

#[derive(Debug, PartialOrd)]
pub struct Vec3 {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Vec3 {
  pub fn new() -> Self {
    Vec3 { x: 0.0, y: 0.0, z: 0.0 }
  }
}

// ! This is not accurate comparison of floating
// ! point numbers.
impl PartialEq for Vec3 {
  fn eq(&self, other: &Self) -> bool {
    (self.x - other.x).abs() < EPS && (self.y - other.y).abs() < EPS && (self.z - other.z).abs() < EPS
  }
}

impl Add for Vec3 {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    }
  }
}

impl Sub for Vec3 {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    Self {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
  }
}

impl Mul<f64> for Vec3 {
  type Output = Self;

  fn mul(self, scalar: f64) -> Self {
    Self {
      x: self.x * scalar,
      y: self.y * scalar,
      z: self.z * scalar,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add() {
    assert_eq!(
      Vec3 { x: 1.0, y: 2.0, z: 3.0 } + Vec3 { x: 4.0, y: 5.0, z: 6.0 },
      Vec3 { x: 5.0, y: 7.0, z: 9.0 }
    );
  }

  #[test]
  fn test_sub() {
    assert_eq!(
      Vec3 { x: 6.0, y: 5.0, z: 4.0 } - Vec3 { x: 1.0, y: 2.0, z: 3.0 },
      Vec3 { x: 5.0, y: 3.0, z: 1.0 }
    );
  }

  #[test]
  fn test_mul_scalar() {
    assert_eq!(
      Vec3 { x: 1.0, y: 2.0, z: 3.0 } * 2.0,
      Vec3 { x: 2.0, y: 4.0, z: 6.0 }
    );
  }
}
