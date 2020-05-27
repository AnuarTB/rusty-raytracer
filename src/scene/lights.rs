use super::objects::Hit;
use crate::geom::Vec3;

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
  fn calculate_intensity(&self, hit: Hit) -> f64;
}

impl Light for PointLight {
  fn calculate_intensity(&self, hit: Hit) -> f64 {
    helper_calc_diffuse(self.intensity, (hit.loc - self.pos).norm(), hit)
  }
}

impl Light for DirectionalLight {
  fn calculate_intensity(&self, hit: Hit) -> f64 {
    helper_calc_diffuse(self.intensity, self.dir.norm(), hit)
  }
}

impl Light for AmbientLight {
  fn calculate_intensity(&self, _hit: Hit) -> f64 {
    self.intensity
  }
}

fn helper_calc_diffuse(intensity: f64, light_vec: Vec3, hit: Hit) -> f64 {
  intensity * (Vec3::dot(light_vec, hit.normal).max(0.0))
}
