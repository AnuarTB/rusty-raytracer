use crate::objects::Hit;
use crate::rendering::Material;
use glm::Vec3;

pub struct PointLight {
  pub intensity: f32,
  pub pos: Vec3,
}

pub struct DirectionalLight {
  pub intensity: f32,
  pub dir: Vec3,
}

pub struct AmbientLight {
  pub intensity: f32,
}

pub enum Light {
  PointLight(PointLight),
  DirectionalLight(DirectionalLight),
  AmbientLight(AmbientLight),
}

impl Light {
  pub fn total_reflection(&self, material: Material, hit: Hit) -> f32 {
    let mut total: f32 = 0.0;
    total += material.diffuse_coeff * self.diffuse_reflection(hit);
    total += material.specular_coeff * self.specular_reflection(hit, material.exp);
    total
  }

  fn diffuse_reflection(&self, hit: Hit) -> f32 {
    match *self {
      Light::PointLight(ref l) => helper_calc_diffuse(l.intensity, glm::normalize(&(l.pos - hit.pos)), hit),
      Light::DirectionalLight(ref l) => helper_calc_diffuse(l.intensity, glm::normalize(&(-l.dir)), hit),
      Light::AmbientLight(ref l) => l.intensity,
    }
  }

  fn specular_reflection(&self, hit: Hit, exp: f32) -> f32 {
    match *self {
      Light::PointLight(ref l) => helper_calc_specular(l.intensity, glm::normalize(&(l.pos - hit.pos)), hit, exp),
      Light::DirectionalLight(ref l) => helper_calc_specular(l.intensity, glm::normalize(&(-l.dir)), hit, exp),
      Light::AmbientLight(ref _l) => 0.0,
    }
  }
}

fn helper_calc_diffuse(intensity: f32, light_vec: Vec3, hit: Hit) -> f32 {
  intensity * (glm::dot(&light_vec, &hit.normal).max(0.0))
}

fn helper_calc_specular(intensity: f32, light_vec: Vec3, hit: Hit, exp: f32) -> f32 {
  // Phong reflection
  let r = 2.0 * (glm::dot(&light_vec, &hit.normal)) * hit.normal - light_vec;
  intensity * glm::dot(&glm::normalize(&r), &glm::normalize(&(-hit.pos))).max(0.0).powf(exp)
}
