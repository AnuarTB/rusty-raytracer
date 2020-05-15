use crate::geom;

pub struct Sphere {
  pub radius: f64,
  pub center: geom::Vec3
}

impl Sphere {
  pub fn new() -> Self {
    Sphere { radius: 0.0, center: geom::Vec3::new() }
  }
}

pub trait SceneObject {
}

impl SceneObject for Sphere {
}