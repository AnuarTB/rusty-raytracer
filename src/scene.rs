// TODO(@anuatb): probably not the best way to import?
use crate::geom::*;
use crate::rendering::Color;

#[derive(Debug)]
pub struct Sphere {
  pub radius: f64,
  pub center: Vec3,
  pub color: Color,
}

impl Sphere {
  pub fn new() -> Self {
    Sphere {
      radius: 0.0,
      center: Vec3::new(),
      color: Color::new(),
    }
  }

  pub fn intersect(&self, orig: Vec3, ray: Vec3) -> Option<f64> {
    // Coefficients of quadratic equation
    let a = ray.len2();
    let b = 2.0 * Vec3::dot(orig - self.center, ray);
    let c = (orig - self.center).len2() - self.radius * self.radius;

    // Computing discriminant
    let d = b * b - 4.0 * a * c;
    if fcmp::smlr(d, 0.0) {
      return None;
    } else {
      let x1 = (-b + d.sqrt()) / (2.0 * a);
      let x2 = (-b - d.sqrt()) / (2.0 * a);
      Some(x1.min(x2))
    }
  }
}
