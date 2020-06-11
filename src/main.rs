use std::fs::File;
use std::io::prelude::*;

use geom::{Ray, Vec3f};
use rendering::{Color, Material};
use scene::lights::*;
use scene::objects::{Hit, Hittable, Sphere};

mod geom;
mod rendering;
mod scene;

fn cast_ray(ray: Ray, objects: &Vec<Sphere>, lights: &Vec<Light>) -> Color {
  let mut nearest = Hit {
    normal: Vec3f::new(),
    loc: Vec3f::new(),
    t: std::f64::INFINITY,
  };
  let mut color_ret = Color { x: 255, y: 255, z: 255 };
  let mut material = Material::new();

  for object in objects {
    match object.hit(ray) {
      None => continue,
      Some(hit) => {
        if geom::fcmp::smlr(hit.t, nearest.t) {
          nearest = hit;
          color_ret = object.material.color;
          material = object.material;
        }
      }
    }
  }

  if nearest.t != std::f64::INFINITY {
    let mut total_intensity = 0.0;
    for light in lights {
      total_intensity += light.total_reflection(material, nearest);
    }
    color_ret = color_ret * total_intensity;
  }
  color_ret
}

fn main() -> std::io::Result<()> {
  // Initialize variables and constants
  const WIDTH: usize = 600;
  const HEIGHT: usize = 600;

  let mut mat = vec![vec![Color::new(); WIDTH]; HEIGHT];
  let mut objects = Vec::new();
  let mut lights: Vec<Light> = Vec::new();
  let origin = Vec3f::new();

  // Scene setup
  objects.push(Sphere {
    radius: 1.0,
    center: Vec3f { x: -1.0, y: 0.0, z: 4.0 },
    material: Material {
      color: Color { x: 210, y: 0, z: 0 },
      diffuse_coeff: 0.6,
      specular_coeff: 0.5,
      exp: 15.0,
    },
  });

  objects.push(Sphere {
    radius: 1.0,
    center: Vec3f { x: 1.0, y: 1.0, z: 5.0 },
    material: Material {
      color: Color { x: 190, y: 255, z: 0 },
      diffuse_coeff: 0.7,
      specular_coeff: 0.2,
      exp: 5.0,
    },
  });

  lights.push(Light::PointLight(PointLight {
    intensity: 1.0,
    pos: Vec3f { x: 0.0, y: -1.0, z: 4.0 },
  }));

  lights.push(Light::DirectionalLight(DirectionalLight {
    intensity: 0.5,
    dir: Vec3f { x: -2.0, y: 0.0, z: 1.0 },
  }));

  lights.push(Light::AmbientLight(AmbientLight { intensity: 0.2 }));

  let mut file = File::create("hello.ppm")?;
  file.write(b"P3\n")?;
  file.write(format!("{} {}\n", &WIDTH, &HEIGHT).as_bytes())?;
  file.write(b"255\n")?;

  for i in 0..HEIGHT {
    for j in 0..WIDTH {
      let height_f = HEIGHT as f64;
      let width_f = WIDTH as f64;
      let y: f64 = (-(i as f64) + height_f / 2.0) / height_f;
      let x: f64 = (j as f64 - width_f / 2.0) / width_f;
      let dir = (Vec3f { x, y, z: 1.0 }).norm();
      mat[i][j] = cast_ray(Ray { orig: origin, dir }, &objects, &lights);
      file.write(format!("{}\t", mat[i][j]).as_bytes())?;
    }
    file.write(b"\n")?;
  }

  Ok(())
}
