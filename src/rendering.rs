use std::fmt;
use std::ops::Mul;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

impl Color {
  pub fn new() -> Color {
    Color { r: 0, g: 0, b: 0 }
  }
}

impl fmt::Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} {} {}", self.r, self.g, self.b)
  }
}

impl Mul<f64> for Color {
  type Output = Self;

  fn mul(self, scalar: f64) -> Self {
    Self {
      r: (self.r as f64 * scalar).min(255.0).max(0.0) as u8,
      g: (self.g as f64 * scalar).min(255.0).max(0.0) as u8,
      b: (self.b as f64 * scalar).min(255.0).max(0.0) as u8,
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct Material {
  pub color: Color,
  pub diffuse_coeff: f64,
  pub specular_coeff: f64,
  pub exp: f64,
}

impl Material {
  pub fn new() -> Self {
    Material {
      color: Color::new(),
      diffuse_coeff: 0.0,
      specular_coeff: 0.0,
      exp: 0.0,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_mul_color() {
    assert_eq!(Color { r: 1, g: 100, b: 30 } * 3.0, Color { r: 3, g: 255, b: 90 });
  }
}
