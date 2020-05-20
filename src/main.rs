use std::fs::File;
use std::io::prelude::*;

use geom::{ Vec3, Ray };
use rendering::Color;
use scene::{ Sphere, Hittable };

mod geom;
mod rendering;
mod scene;

fn cast_ray(ray: Ray, objects: &Vec<Sphere>) -> Color {
  let mut nearest_dist: f64 = f64::INFINITY;
  let mut color_ret = Color { r: 255, g: 255, b: 255 };
  for object in objects {
    match object.hit(ray) {
      None => continue,
      Some(hit) => {
        if geom::fcmp::smlr(hit.t, nearest_dist) {
          nearest_dist = hit.t;
          color_ret = object.color;
        }
      }
    }
  }
  color_ret
}

fn main() -> std::io::Result<()> {
  // Initialize variables and constants
  const WIDTH: usize = 600;
  const HEIGHT: usize = 600;

  let mut mat = vec![vec![Color::new(); WIDTH]; HEIGHT];
  let mut objects = Vec::new();
  let origin = Vec3::new();

  // Scene setup
  objects.push(Sphere {
    radius: 1.0,
    center: Vec3 { x: 0.0, y: 0.0, z: 3.0 },
    color: Color { r: 255, g: 0, b: 0 },
  });

  objects.push(Sphere {
    radius: 1.0,
    center: Vec3 { x: 1.0, y: 1.0, z: 4.0 },
    color: Color { r: 255, g: 255, b: 0 },
  });

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
      let dir = (Vec3 { x, y, z: 1.0 }).norm();
      mat[i][j] = cast_ray(Ray { orig: origin, dir }, &objects);
      file.write(format!("{}\t", mat[i][j]).as_bytes())?;
    }
    file.write(b"\n")?;
  }

  Ok(())
}
