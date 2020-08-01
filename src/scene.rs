use crate::geom::Ray;
use crate::lights::Light;
use crate::objects::Hittable;
use crate::rendering::{cast_ray, Color};
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::prelude::*;

use glm::{Vec3, Vec4};
use rayon::prelude::*;

pub struct Camera {
  pub fov: f32,
  pub camera_pos: Vec3,
  pub look_at: Vec3,
  pub num_samples: usize,
}

impl Camera {
  pub fn new(fov: f32, camera_pos: Vec3, look_at: Vec3, num_samples: usize) -> Self {
    Self {
      fov,
      camera_pos,
      look_at,
      num_samples,
    }
  }
}

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
  pub camera: Camera,

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
      camera: Camera::new(fov, glm::zero(), Vec3::new(0.0, 0.0, 1.0), 30),
      recursion_depth,
    }
  }

  pub fn update(&mut self) {
    let aspect_ratio: f32 = (self.width as f32) / (self.height as f32);
    let fov_adjustment = (self.camera.fov.to_radians() / 2.0).tan();
    let look_at_mat = glm::look_at(&self.camera.camera_pos, &self.camera.look_at, &Vec3::new(0.0, 1.0, 0.0));

    let cartesian = (0..self.height)
      .flat_map(|y| (0..self.width).clone().map(move |x| (y, x)))
      .collect::<Vec<(usize, usize)>>()
      .into_par_iter();

    let height_f = self.height as f32;
    let width_f = self.width as f32;

    self.framebuffer = cartesian
      .map(|(i, j)| {
        let mut total: Color = glm::zero();
        let mut rng = thread_rng();

        for _ in 0..self.camera.num_samples {
          let x: f32 = (1.0 - (j as f32 + 0.5 + rng.gen::<f32>()) / width_f * 2.0) * fov_adjustment * aspect_ratio;
          let y: f32 = (1.0 - (i as f32 + 0.5 + rng.gen::<f32>()) / height_f * 2.0) * fov_adjustment;
          let dir = glm::vec4_to_vec3(&(look_at_mat * Vec4::new(x, y, -1.0, 1.0)));
          total += cast_ray(
            Ray {
              orig: self.camera.camera_pos,
              dir,
            },
            &self.objects,
            &self.lights,
            self.recursion_depth,
          );
        }
        total * (1.0 / (self.camera.num_samples as f32))
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
