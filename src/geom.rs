use glm::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
  pub orig: Vec3,
  pub dir: Vec3,
}

impl Ray {
  pub fn new_norm(orig: Vec3, dir: Vec3) -> Self {
    Ray {
      orig,
      dir: glm::normalize(&dir),
    }
  }

  pub fn at(&self, t: f32) -> Vec3 {
    self.orig + self.dir * t
  }
}
