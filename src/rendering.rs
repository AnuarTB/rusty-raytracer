use crate::geom::*;
use crate::lights::Light;
use crate::objects::*;
use glm::Vec3;

pub type Color = Vec3;

// Constants
lazy_static! {
  static ref BACKGROUND_COLOR: Color = Color::new(1.0, 1.0, 1.0);
}

const SHADOW_BIAS: f32 = 1e-4;

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
      color: glm::zero(),
      diffuse_coeff: 1.0,
      specular_coeff: 0.0,
      exp: 0.0,
      refl: 0.0,
    }
  }
}

pub fn hit_object(ray: Ray, objects: &Vec<Sphere>) -> Option<(Hit, &Sphere)> {
  let mut nearest: Option<(Hit, &Sphere)> = None;
  for object in objects {
    match object.hit(ray) {
      None => continue,
      Some(hit) => {
        if nearest.is_none() || hit.t < nearest.unwrap().0.t {
          nearest = Some((hit, &object));
        }
      }
    }
  }
  nearest
}

pub fn cast_ray(ray: Ray, objects: &Vec<Sphere>, lights: &Vec<Light>, depth: u32) -> Color {
  match hit_object(ray, objects) {
    None => *BACKGROUND_COLOR,
    Some((hit, object)) => {
      let mut total_intensity: f32 = 0.0;
      for light in lights {
        let in_shadow: bool = match light {
          Light::PointLight(l) => {
            let shadow_hit = hit_object(Ray::new_norm(hit.pos + (hit.normal * SHADOW_BIAS), l.pos - hit.pos), objects);
            !(shadow_hit.is_none() || shadow_hit.unwrap().0.t > glm::length(&(l.pos - hit.pos)))
          }
          _ => false,
        };
        if !in_shadow {
          total_intensity += light.total_reflection(object.material, hit);
        }
      }
      let ret: Vec3;
      if object.material.refl > 0.0 && depth > 0 {
        ret = object.material.color * total_intensity * (1.0 - object.material.refl)
          + cast_ray(
            Ray::new_norm(hit.pos + hit.normal * SHADOW_BIAS, glm::reflect_vec(&ray.dir, &hit.normal)),
            objects,
            lights,
            depth - 1,
          ) * object.material.refl;
      } else {
        ret = object.material.color * total_intensity;
      }

      glm::clamp(&ret, 0.0, 1.0)
    }
  }
}
