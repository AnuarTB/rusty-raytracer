use crate::geom::*;
use crate::lights::Light;
use crate::objects::*;

use std::fmt;
use std::ops::Mul;

pub type Color = Vec3<u8>;

impl Color {
  pub fn new() -> Color {
    Color { x: 0, y: 0, z: 0 }
  }
}

impl fmt::Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} {} {}", self.x, self.y, self.z)
  }
}

impl Mul<f64> for Color {
  type Output = Self;

  fn mul(self, scalar: f64) -> Self {
    Self {
      x: (self.x as f64 * scalar).min(255.0).max(0.0) as u8,
      y: (self.y as f64 * scalar).min(255.0).max(0.0) as u8,
      z: (self.z as f64 * scalar).min(255.0).max(0.0) as u8,
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
    assert_eq!(Color { x: 1, y: 100, z: 30 } * 3.0, Color { x: 3, y: 255, z: 90 });
  }
}

pub fn cast_ray(ray: Ray, objects: &Vec<Sphere>, lights: &Vec<Light>) -> Color {
  let mut nearest = Hit {
    normal: Vec3f::new(),
    loc: Vec3f::new(),
    t: std::f64::INFINITY,
  };
  let mut color_ret = Color { x: 255, y: 255, z: 255 };
  let mut material = Material::new();

  for object in objects {
    match object.hit(ray) {
      None => continue,
      Some(hit) => {
        if fcmp::smlr(hit.t, nearest.t) {
          nearest = hit;
          color_ret = object.material.color;
          material = object.material;
        }
      }
    }
  }

  if nearest.t != std::f64::INFINITY {
    let mut total_intensity = 0.0;
    for light in lights {
      total_intensity += light.total_reflection(material, nearest);
    }
    color_ret = color_ret * total_intensity;
  }
  color_ret
}
