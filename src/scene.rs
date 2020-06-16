use crate::geom::{Ray, Vec3f};
use crate::lights::Light;
use crate::objects::Sphere;
use crate::rendering::{cast_ray, Color};
use std::fs::File;
use std::io::prelude::*;

const ORIGIN: Vec3f = Vec3f { x: 0.0, y: 0.0, z: 0.0 };

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
  pub fov: f64,
}

impl Scene {
  pub fn new(width: usize, height: usize, fov: f64) -> Self {
    Self {
      objects: Vec::new(),
      lights: Vec::new(),
      width,
      height,
      framebuffer: vec![vec![Color::new(); width]; height],
      fov,
    }
  }

  pub fn update(&mut self) {
    let aspect_ratio: f64 = (self.width as f64) / (self.height as f64);
    let fov_adjustment = (self.fov.to_radians() / 2.0).tan();
    for i in 0..self.height {
      for j in 0..self.width {
        let height_f = self.height as f64;
        let width_f = self.width as f64;
        let x: f64 = ((j as f64 + 0.5) / width_f * 2.0 - 1.0) * fov_adjustment * aspect_ratio;
        let y: f64 = (1.0 - (i as f64 + 0.5) / height_f * 2.0) * fov_adjustment;
        let dir = (Vec3f { x, y, z: 1.0 }).norm();
        self.framebuffer[i][j] = cast_ray(Ray { orig: ORIGIN, dir }, &self.objects, &self.lights);
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
