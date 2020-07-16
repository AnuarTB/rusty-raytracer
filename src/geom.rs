use glm::Vec3;
/*
#[derive(Debug, Copy, Clone)]
pub struct Vec3<T> {
  pub x: T,
  pub y: T,
  pub z: T,
}

pub type Vec3 = Vec3<f32>;
*/

pub mod fcmp {
  const EPS: f32 = 1e-12;

  /// Equal
  pub fn eql(a: f32, b: f32) -> bool {
    (a - b).abs() < EPS
  }

  /// a is smaller than b?
  pub fn smlr(a: f32, b: f32) -> bool {
    (b - a) >= EPS
  }

  /// a is greater than b?
  pub fn grtr(a: f32, b: f32) -> bool {
    (a - b) >= EPS
  }
}

/*
impl Vec3 {
  pub fn new() -> Self {
    Vec3 { x: 0.0, y: 0.0, z: 0.0 }
  }

  pub fn dot(a: Vec3, b: Vec3) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
  }

  pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
      x: a.y * b.z - a.z * b.y,
      y: a.z * b.x - a.x * b.z,
      z: a.x * b.y - a.y * b.x,
    }
  }

  pub fn reflect(&self, normal: Vec3) -> Vec3 {
    2.0 * glm::dot(*self, normal.norm()) * normal.norm() - *self
  }

  /// Returns a squared length of the vector
  pub fn len2(&self) -> f32 {
    self.x * self.x + self.y * self.y + self.z * self.z
  }

  pub fn len(&self) -> f32 {
    self.len2().sqrt()
  }

  /// Returns new normalized version of the vector
  pub fn norm(&self) -> Self {
    let len: f32 = self.len();
    Vec3 {
      x: self.x / len,
      y: self.y / len,
      z: self.z / len,
    }
  }
}

// ! This is not accurate comparison of floating
// ! point numbers.
impl PartialEq for Vec3 {
  fn eq(&self, other: &Self) -> bool {
    fcmp::eql(self.x, other.x) && fcmp::eql(self.y, other.y) && fcmp::eql(self.z, other.z)
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

impl Mul<f32> for Vec3 {
  type Output = Self;

  fn mul(self, scalar: f32) -> Self {
    Self {
      x: self.x * scalar,
      y: self.y * scalar,
      z: self.z * scalar,
    }
  }
}

impl Mul<Vec3> for f32 {
  type Output = Vec3;

  fn mul(self, vec: Vec3) -> Vec3 {
    Vec3 {
      x: self * vec.x,
      y: self * vec.y,
      z: self * vec.z,
    }
  }
}

impl Neg for Vec3 {
  type Output = Self;

  fn neg(self) -> Self {
    Self {
      x: -self.x,
      y: -self.y,
      z: -self.z,
    }
  }
}
*/

#[derive(Debug, Copy, Clone)]
pub struct Ray {
  pub orig: Vec3,
  pub dir: Vec3,
}

impl Ray {
  pub fn new_norm(orig: Vec3, dir: Vec3) -> Self {
    Ray {
      orig,
      dir: glm::normalize(&dir),
    }
  }

  pub fn at(&self, t: f32) -> Vec3 {
    self.orig + self.dir * t
  }
}

/*
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_neg() {
    assert_eq!(-Vec3 { x: 1.0, y: 2.0, z: 3.0 }, Vec3 { x: -1.0, y: -2.0, z: -3.0 });
  }

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
    assert_eq!(Vec3 { x: 1.0, y: 2.0, z: 3.0 } * 2.0, Vec3 { x: 2.0, y: 4.0, z: 6.0 });
    assert_eq!(2.0 * Vec3 { x: 1.0, y: 2.0, z: 3.0 }, Vec3 { x: 2.0, y: 4.0, z: 6.0 });
  }

  #[test]
  fn test_len() {
    let v = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
    assert!(fcmp::eql(v.len2(), 14.0));
  }

  #[test]
  fn test_norm() {
    let v = (Vec3 { x: 1.0, y: 2.0, z: 3.0 }).norm();
    assert!(fcmp::eql(v.len(), 1.0));
  }

  #[test]
  fn test_dot() {
    assert!(fcmp::eql(
      glm::dot(Vec3 { x: 1.0, y: 2.0, z: 3.0 }, Vec3 { x: 4.0, y: 5.0, z: 6.0 }),
      32.0
    ));
  }

  #[test]
  fn test_cross() {
    assert_eq!(
      Vec3::cross(Vec3 { x: 1.0, y: 2.0, z: 3.0 }, Vec3 { x: 4.0, y: 5.0, z: 6.0 }),
      Vec3 { x: -3.0, y: 6.0, z: -3.0 }
    )
  }
}
*/
