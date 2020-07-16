use crate::geom::Ray;
use crate::lights::Light;
use crate::objects::Sphere;
use crate::rendering::{cast_ray, Color};
use std::fs::File;
use std::io::prelude::*;

use glm::Vec3;

pub struct Scene {
  // TODO: Separate objects
  // Scene objects
  pub objects: Vec<Sphere>,
  pub lights: Vec<Light>,

  // Viewport
  pub width: usize,
  pub height: usize,
  pub framebuffer: Vec<Vec<Color>>,

  // Camera
  pub fov: f32,

  // Rendering
  pub recursion_depth: u32,
}

impl Scene {
  pub fn new(width: usize, height: usize, fov: f32, recursion_depth: u32) -> Self {
    Self {
      objects: Vec::new(),
      lights: Vec::new(),
      width,
      height,
      framebuffer: vec![vec![Color::new(); width]; height],
      fov,
      recursion_depth,
    }
  }

  pub fn update(&mut self) {
    let aspect_ratio: f32 = (self.width as f32) / (self.height as f32);
    let fov_adjustment = (self.fov.to_radians() / 2.0).tan();
    for i in 0..self.height {
      for j in 0..self.width {
        let height_f = self.height as f32;
        let width_f = self.width as f32;
        let x: f32 = ((j as f32 + 0.5) / width_f * 2.0 - 1.0) * fov_adjustment * aspect_ratio;
        let y: f32 = (1.0 - (i as f32 + 0.5) / height_f * 2.0) * fov_adjustment;
        let dir = glm::normalize(&Vec3::new(x, y, 1.0));
        self.framebuffer[i][j] = cast_ray(Ray { orig: glm::zero(), dir }, &self.objects, &self.lights, self.recursion_depth);
      }
    }
  }

  pub fn render_to_ppm(&mut self, filename: &str) -> std::io::Result<()> {
    self.update();

    let mut file = File::create(filename)?;

    file.write(b"P3\n")?;
    file.write(format!("{} {}\n", &self.width, &self.height).as_bytes())?;
    file.write(b"255\n")?;

    for i in 0..self.height {
      for j in 0..self.width {
        file.write(format!("{}\t", self.framebuffer[i][j]).as_bytes())?;
      }
      file.write(b"\n")?;
    }

    Ok(())
  }
}
