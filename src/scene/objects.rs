// TODO(@anuatb): probably not the best way to import?
use crate::geom::*;
use crate::rendering::Material;

#[derive(Debug, Clone, Copy)]
pub struct Hit {
  pub normal: Vec3,
  pub loc: Vec3,
  pub t: f64,
}

pub trait Hittable {
  // Return Hit object if the object was hitted by ray.
  fn hit(&self, ray: Ray) -> Option<Hit>;
}

#[derive(Debug)]
pub struct Sphere {
  pub radius: f64,
  pub center: Vec3,
  pub material: Material,
}

impl Sphere {
  pub fn new() -> Self {
    Sphere {
      radius: 0.0,
      center: Vec3::new(),
      material: Material::new(),
    }
  }
}

impl Hittable for Sphere {
  fn hit(&self, ray: Ray) -> Option<Hit> {
    // Coefficients of quadratic equation
    let a = ray.dir.len2();
    let b = 2.0 * Vec3::dot(ray.orig - self.center, ray.dir);
    let c = (ray.orig - self.center).len2() - self.radius * self.radius;

    // Computing discriminant
    let d = b * b - 4.0 * a * c;
    if fcmp::smlr(d, 0.0) {
      return None;
    } else {
      let x1 = (-b + d.sqrt()) / (2.0 * a);
      let x2 = (-b - d.sqrt()) / (2.0 * a);

      let t = x1.min(x2);
      let normal = (ray.at(t) - self.center).norm();
      let loc = ray.at(t);

      Some(Hit { normal, loc, t })
    }
  }
}
