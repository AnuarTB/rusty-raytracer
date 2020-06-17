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

impl PartialEq for Color {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == other.y && self.z == other.z
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

const BACKGROUND_COLOR: Color = Color { x: 255, y: 255, z: 255 };
const SHADOW_BIAS: f64 = 1e-4;

pub fn hit_object(ray: Ray, objects: &Vec<Sphere>) -> Option<(Hit, &Sphere)> {
  let mut nearest: Option<(Hit, &Sphere)> = None;
  for object in objects {
    match object.hit(ray) {
      None => continue,
      Some(hit) => {
        if nearest.is_none() || fcmp::smlr(hit.t, nearest.unwrap().0.t) {
          nearest = Some((hit, &object));
        }
      }
    }
  }
  nearest
}

pub fn cast_ray(ray: Ray, objects: &Vec<Sphere>, lights: &Vec<Light>) -> Color {
  match hit_object(ray, objects) {
    None => BACKGROUND_COLOR,
    Some((hit, object)) => {
      let mut total_intensity: f64 = 0.0;
      for light in lights {
        let in_shadow: bool = match light {
          Light::PointLight(l) => {
            let shadow_hit = hit_object(Ray::new_norm(hit.pos + (hit.normal * SHADOW_BIAS), l.pos - hit.pos), objects);
            !(shadow_hit.is_none() || fcmp::grtr(shadow_hit.unwrap().0.t, (l.pos - hit.pos).len()))
          }
          _ => false,
        };
        if !in_shadow {
          total_intensity += light.total_reflection(object.material, hit);
        }
      }
      object.material.color * total_intensity
    }
  }
}
