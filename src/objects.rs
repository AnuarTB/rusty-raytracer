// TODO(@anuatb): probably not the best way to import?
use crate::geom::*;
use crate::rendering::Material;

use glm::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Hit<'a> {
  pub normal: Vec3,
  pub pos: Vec3,
  pub t: f32,
  pub material: &'a Material,
}

#[derive(Debug, Clone, Copy)]
pub struct AABB {
  pub mn: Vec3,
  pub mx: Vec3,
}

impl Default for AABB {
  fn default() -> Self {
    Self {
      mn: glm::zero(),
      mx: glm::zero(),
    }
  }
}

pub trait Hittable {
  // Return Hit object if the object was hitted by ray.
  fn hit(&self, ray: Ray) -> Option<Hit>;

  fn update_bbox(&mut self) {}

  fn aabb(&self, ray: Ray) -> bool {
    true
  }
}

#[derive(Debug, Clone, Copy, Builder)]
pub struct Sphere {
  pub radius: f32,
  pub center: Vec3,
  pub material: Material,
}

impl Default for Sphere {
  fn default() -> Self {
    Sphere {
      radius: 0.0,
      center: glm::zero(),
      material: Material::default(),
    }
  }
}

impl Hittable for Sphere {
  fn hit(&self, ray: Ray) -> Option<Hit> {
    // Coefficients of quadratic equation
    let a = glm::length2(&ray.dir);
    let b = 2.0 * glm::dot(&(ray.orig - self.center), &ray.dir);
    let c = glm::length2(&(ray.orig - self.center)) - self.radius * self.radius;

    // Computing discriminant
    let d = b * b - 4.0 * a * c;
    if d < 0.0 {
      return None;
    } else {
      let x1 = (-b + d.sqrt()) / (2.0 * a);
      let x2 = (-b - d.sqrt()) / (2.0 * a);

      let mut t = x1.min(x2);

      if x1 < 0.0 || x2 < 0.0 {
        if x1 < 0.0 && x1 < 0.0 {
          return None;
        } else if x1 < 0.0 {
          t = x2;
        } else {
          t = x1;
        }
      }

      let normal = glm::normalize(&(ray.at(t) - self.center));
      let pos = ray.at(t);

      Some(Hit {
        normal,
        pos,
        t,
        material: &self.material,
      })
    }
  }
}
