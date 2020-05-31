use super::objects::Hit;

use crate::geom::Vec3;
use crate::rendering::Material;

pub struct PointLight {
  pub intensity: f64,
  pub pos: Vec3,
}

pub struct DirectionalLight {
  pub intensity: f64,
  pub dir: Vec3,
}

pub struct AmbientLight {
  pub intensity: f64,
}

pub trait Light {
  fn total_reflection(&self, material: Material, hit: Hit) -> f64 {
    let mut total: f64 = 0.0;
    total += material.diffuse_coeff * self.diffuse_reflection(hit);
    total += material.specular_coeff * self.specular_reflection(hit, material.exp);
    total
  }

  fn diffuse_reflection(&self, _hit: Hit) -> f64 {
    0.0
  }

  fn specular_reflection(&self, _hit: Hit, _exp: f64) -> f64 {
    0.0
  }
}

impl Light for PointLight {
  fn diffuse_reflection(&self, hit: Hit) -> f64 {
    helper_calc_diffuse(self.intensity, (hit.loc - self.pos).norm(), hit)
  }

  fn specular_reflection(&self, hit: Hit, exp: f64) -> f64 {
    helper_calc_specular(self.intensity, (hit.loc - self.pos).norm(), hit, exp)
  }
}

impl Light for DirectionalLight {
  fn diffuse_reflection(&self, hit: Hit) -> f64 {
    helper_calc_diffuse(self.intensity, self.dir.norm(), hit)
  }

  fn specular_reflection(&self, hit: Hit, exp: f64) -> f64 {
    helper_calc_specular(self.intensity, self.dir.norm(), hit, exp)
  }
}

impl Light for AmbientLight {
  fn diffuse_reflection(&self, _hit: Hit) -> f64 {
    self.intensity
  }
}

fn helper_calc_diffuse(intensity: f64, light_vec: Vec3, hit: Hit) -> f64 {
  intensity * (Vec3::dot(light_vec, hit.normal).max(0.0))
}

fn helper_calc_specular(intensity: f64, light_vec: Vec3, hit: Hit, exp: f64) -> f64 {
  // Phong reflection
  let r = 2.0 * (Vec3::dot(light_vec, hit.normal)) * hit.normal - light_vec;
  intensity * Vec3::dot(r.norm(), -hit.loc.norm()).max(0.0).powf(exp)
}
