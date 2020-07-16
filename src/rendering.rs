use crate::geom::*;
use crate::lights::Light;
use crate::objects::*;

use std::fmt;
use std::ops::{Add, Mul};

#[derive(Copy, Clone, Debug)]
pub struct Color {
  pub x: u8,
  pub y: u8,
  pub z: u8,
}

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

impl Mul<f32> for Color {
  type Output = Self;

  fn mul(self, scalar: f32) -> Self {
    Self {
      x: (self.x as f32 * scalar).min(255.0).max(0.0) as u8,
      y: (self.y as f32 * scalar).min(255.0).max(0.0) as u8,
      z: (self.z as f32 * scalar).min(255.0).max(0.0) as u8,
    }
  }
}

impl Mul<Color> for f32 {
  type Output = Color;

  fn mul(self, other: Color) -> Color {
    Color {
      x: (other.x as f32 * self).min(255.0).max(0.0) as u8,
      y: (other.y as f32 * self).min(255.0).max(0.0) as u8,
      z: (other.z as f32 * self).min(255.0).max(0.0) as u8,
    }
  }
}

impl Add<Color> for Color {
  type Output = Self;

  fn add(self, other: Color) -> Self {
    Self {
      x: (self.x as u16 + other.x as u16).min(255).max(0) as u8,
      y: (self.y as u16 + other.y as u16).min(255).max(0) as u8,
      z: (self.z as u16 + other.z as u16).min(255).max(0) as u8,
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct Material {
  pub color: Color,
  pub diffuse_coeff: f32,
  pub specular_coeff: f32,
  pub exp: f32,
  pub refl: f32,
}

impl Material {
  pub fn new() -> Self {
    Material {
      color: Color::new(),
      diffuse_coeff: 0.0,
      specular_coeff: 0.0,
      exp: 0.0,
      refl: 0.0,
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
const SHADOW_BIAS: f32 = 1e-4;

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

pub fn cast_ray(ray: Ray, objects: &Vec<Sphere>, lights: &Vec<Light>, depth: u32) -> Color {
  match hit_object(ray, objects) {
    None => BACKGROUND_COLOR,
    Some((hit, object)) => {
      let mut total_intensity: f32 = 0.0;
      for light in lights {
        let in_shadow: bool = match light {
          Light::PointLight(l) => {
            let shadow_hit = hit_object(Ray::new_norm(hit.pos + (hit.normal * SHADOW_BIAS), l.pos - hit.pos), objects);
            !(shadow_hit.is_none() || fcmp::grtr(shadow_hit.unwrap().0.t, glm::length(&(l.pos - hit.pos))))
          }
          _ => false,
        };
        if !in_shadow {
          total_intensity += light.total_reflection(object.material, hit);
        }
      }

      if object.material.refl > 0.0 && depth > 0 {
        object.material.color * total_intensity * (1.0 - object.material.refl)
          + cast_ray(
            Ray::new_norm(hit.pos + hit.normal * SHADOW_BIAS, glm::reflect_vec(&ray.dir, &hit.normal)),
            objects,
            lights,
            depth - 1,
          ) * object.material.refl
      } else {
        object.material.color * total_intensity
      }
    }
  }
}
