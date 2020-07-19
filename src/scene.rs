use crate::geom::Ray;
use crate::lights::Light;
use crate::objects::Hittable;
use crate::rendering::{cast_ray, Color};
use std::fs::File;
use std::io::prelude::*;

use glm::{Vec3, Vec4};
use rayon::prelude::*;

pub struct Scene {
  // TODO: Separate objects
  // Scene objects
  pub objects: Vec<Box<dyn Hittable + Send + Sync>>,
  pub lights: Vec<Light>,

  // Viewport
  pub width: usize,
  pub height: usize,
  pub framebuffer: Vec<Color>,

  // Camera
  pub fov: f32,
  pub camera_pos: Vec3,
  pub look_at: Vec3,

  // Rendering
  pub recursion_depth: u32,
}

impl<'a> Scene {
  pub fn new(width: usize, height: usize, fov: f32, recursion_depth: u32) -> Self {
    Self {
      objects: Vec::new(),
      lights: Vec::new(),
      width,
      height,
      framebuffer: vec![glm::zero(); width * height],
      fov,
      camera_pos: glm::zero(),
      look_at: Vec3::new(0.0, 0.0, 1.0),
      recursion_depth,
    }
  }

  pub fn update(&mut self) {
    let aspect_ratio: f32 = (self.width as f32) / (self.height as f32);
    let fov_adjustment = (self.fov.to_radians() / 2.0).tan();
    let look_at_mat = glm::look_at(&self.camera_pos, &self.look_at, &Vec3::new(0.0, 1.0, 0.0));

    let cartesian = (0..self.height)
      .flat_map(|y| (0..self.width).clone().map(move |x| (y, x)))
      .collect::<Vec<(usize, usize)>>()
      .into_par_iter();

    let height_f = self.height as f32;
    let width_f = self.width as f32;

    self.framebuffer = cartesian
      .map(|(i, j)| {
        let x: f32 = (1.0 - (j as f32 + 0.5) / width_f * 2.0) * fov_adjustment * aspect_ratio;
        let y: f32 = (1.0 - (i as f32 + 0.5) / height_f * 2.0) * fov_adjustment;

        let dir = glm::vec4_to_vec3(&(look_at_mat * Vec4::new(x, y, -1.0, 1.0)));

        cast_ray(
          Ray {
            orig: self.camera_pos,
            dir,
          },
          &self.objects,
          &self.lights,
          self.recursion_depth,
        )
      })
      .collect();
  }

  pub fn convert_color256(color: Color) -> String {
    let ret: Vec3 = color * 255.0;
    format!("{} {} {}", ret.x as u8, ret.y as u8, ret.z as u8)
  }

  pub fn render_to_ppm(&mut self, filename: &str) -> std::io::Result<()> {
    self.update();

    let mut file = File::create(filename)?;

    file.write(b"P3\n")?;
    file.write(format!("{} {}\n", &self.width, &self.height).as_bytes())?;
    file.write(b"255\n")?;

    for i in 0..(self.height * self.width) {
      file.write(format!("{}\n", Scene::convert_color256(self.framebuffer[i])).as_bytes())?;
    }

    Ok(())
  }
}
