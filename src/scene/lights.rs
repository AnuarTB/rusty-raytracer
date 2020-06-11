use super::objects::Hit;

use crate::geom::Vec3f;
use crate::rendering::Material;

pub struct PointLight {
  pub intensity: f64,
  pub pos: Vec3f,
}

pub struct DirectionalLight {
  pub intensity: f64,
  pub dir: Vec3f,
}

pub struct AmbientLight {
  pub intensity: f64,
}

pub enum Light {
  PointLight(PointLight),
  DirectionalLight(DirectionalLight),
  AmbientLight(AmbientLight),
}

impl Light {
  pub fn total_reflection(&self, material: Material, hit: Hit) -> f64 {
    let mut total: f64 = 0.0;
    total += material.diffuse_coeff * self.diffuse_reflection(hit);
    total += material.specular_coeff * self.specular_reflection(hit, material.exp);
    total
  }

  fn diffuse_reflection(&self, hit: Hit) -> f64 {
    match *self {
      Light::PointLight(ref l) => helper_calc_diffuse(l.intensity, (hit.loc - l.pos).norm(), hit),
      Light::DirectionalLight(ref l) => helper_calc_diffuse(l.intensity, l.dir.norm(), hit),
      Light::AmbientLight(ref l) => l.intensity,
    }
  }

  fn specular_reflection(&self, hit: Hit, exp: f64) -> f64 {
    match *self {
      Light::PointLight(ref l) => helper_calc_specular(l.intensity, (hit.loc - l.pos).norm(), hit, exp),
      Light::DirectionalLight(ref l) => helper_calc_specular(l.intensity, l.dir.norm(), hit, exp),
      Light::AmbientLight(ref l) => 0.0,
    }
  }
}

fn helper_calc_diffuse(intensity: f64, light_vec: Vec3f, hit: Hit) -> f64 {
  intensity * (Vec3f::dot(light_vec, hit.normal).max(0.0))
}

fn helper_calc_specular(intensity: f64, light_vec: Vec3f, hit: Hit, exp: f64) -> f64 {
  // Phong reflection
  let r = 2.0 * (Vec3f::dot(light_vec, hit.normal)) * hit.normal - light_vec;
  intensity * Vec3f::dot(r.norm(), -hit.loc.norm()).max(0.0).powf(exp)
}
