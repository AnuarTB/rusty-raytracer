use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vec3<T> {
  pub x: T,
  pub y: T,
  pub z: T,
}

pub type Vec3f = Vec3<f64>;

pub mod fcmp {
  const EPS: f64 = 1e-12;

  /// Equal
  pub fn eql(a: f64, b: f64) -> bool {
    (a - b).abs() < EPS
  }

  /// a is smaller than b?
  pub fn smlr(a: f64, b: f64) -> bool {
    (b - a) >= EPS
  }

  /// a is greater than b?
  pub fn grtr(a: f64, b: f64) -> bool {
    (a - b) >= EPS
  }
}

impl Vec3f {
  pub fn new() -> Self {
    Vec3f { x: 0.0, y: 0.0, z: 0.0 }
  }

  pub fn dot(a: Vec3f, b: Vec3f) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z
  }

  pub fn cross(a: Vec3f, b: Vec3f) -> Vec3f {
    Vec3f {
      x: a.y * b.z - a.z * b.y,
      y: a.z * b.x - a.x * b.z,
      z: a.x * b.y - a.y * b.x,
    }
  }

  /// Returns a squared length of the vector
  pub fn len2(&self) -> f64 {
    self.x * self.x + self.y * self.y + self.z * self.z
  }

  pub fn len(&self) -> f64 {
    self.len2().sqrt()
  }

  /// Returns new normalized version of the vector
  pub fn norm(&self) -> Self {
    let len: f64 = self.len();
    Vec3f {
      x: self.x / len,
      y: self.y / len,
      z: self.z / len,
    }
  }
}

// ! This is not accurate comparison of floating
// ! point numbers.
impl PartialEq for Vec3f {
  fn eq(&self, other: &Self) -> bool {
    fcmp::eql(self.x, other.x) && fcmp::eql(self.y, other.y) && fcmp::eql(self.z, other.z)
  }
}

impl<T: Add<Output = T>> Add for Vec3<T> {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    }
  }
}

impl<T: Sub<Output = T>> Sub for Vec3<T> {
  type Output = Self;

  fn sub(self, other: Self) -> Self::Output {
    Self {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
  }
}

impl Mul<f64> for Vec3f {
  type Output = Self;

  fn mul(self, scalar: f64) -> Self {
    Self {
      x: self.x * scalar,
      y: self.y * scalar,
      z: self.z * scalar,
    }
  }
}

impl Mul<Vec3f> for f64 {
  type Output = Vec3f;

  fn mul(self, vec: Vec3f) -> Vec3f {
    Vec3f {
      x: self * vec.x,
      y: self * vec.y,
      z: self * vec.z,
    }
  }
}

impl Neg for Vec3f {
  type Output = Self;

  fn neg(self) -> Self {
    Self {
      x: -self.x,
      y: -self.y,
      z: -self.z,
    }
  }
}

#[derive(Debug, Copy, Clone)]
pub struct Ray {
  pub orig: Vec3f,
  pub dir: Vec3f,
}

impl Ray {
  pub fn new_norm(orig: Vec3f, dir: Vec3f) -> Self {
    Ray { orig, dir: dir.norm() }
  }

  pub fn at(&self, t: f64) -> Vec3f {
    self.orig + self.dir * t
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_neg() {
    assert_eq!(-Vec3f { x: 1.0, y: 2.0, z: 3.0 }, Vec3f { x: -1.0, y: -2.0, z: -3.0 });
  }

  #[test]
  fn test_add() {
    assert_eq!(
      Vec3f { x: 1.0, y: 2.0, z: 3.0 } + Vec3f { x: 4.0, y: 5.0, z: 6.0 },
      Vec3f { x: 5.0, y: 7.0, z: 9.0 }
    );
  }

  #[test]
  fn test_sub() {
    assert_eq!(
      Vec3f { x: 6.0, y: 5.0, z: 4.0 } - Vec3f { x: 1.0, y: 2.0, z: 3.0 },
      Vec3f { x: 5.0, y: 3.0, z: 1.0 }
    );
  }

  #[test]
  fn test_mul_scalar() {
    assert_eq!(Vec3f { x: 1.0, y: 2.0, z: 3.0 } * 2.0, Vec3f { x: 2.0, y: 4.0, z: 6.0 });
    assert_eq!(2.0 * Vec3f { x: 1.0, y: 2.0, z: 3.0 }, Vec3f { x: 2.0, y: 4.0, z: 6.0 });
  }

  #[test]
  fn test_len() {
    let v = Vec3f { x: 1.0, y: 2.0, z: 3.0 };
    assert!(fcmp::eql(v.len2(), 14.0));
  }

  #[test]
  fn test_norm() {
    let v = (Vec3f { x: 1.0, y: 2.0, z: 3.0 }).norm();
    assert!(fcmp::eql(v.len(), 1.0));
  }

  #[test]
  fn test_dot() {
    assert!(fcmp::eql(
      Vec3f::dot(Vec3f { x: 1.0, y: 2.0, z: 3.0 }, Vec3f { x: 4.0, y: 5.0, z: 6.0 }),
      32.0
    ));
  }

  #[test]
  fn test_cross() {
    assert_eq!(
      Vec3f::cross(Vec3f { x: 1.0, y: 2.0, z: 3.0 }, Vec3f { x: 4.0, y: 5.0, z: 6.0 }),
      Vec3f { x: -3.0, y: 6.0, z: -3.0 }
    )
  }
}
