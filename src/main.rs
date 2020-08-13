extern crate nalgebra_glm as glm;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate derive_builder;

use glm::Vec3;
use lights::*;
use obj::ObjBuilder;
use objects::{Hittable, Sphere};
use rendering::{Color, Material, MaterialBuilder};
use scene::{CameraBuilder, SceneBuilder};

mod geom;
mod lights;
mod obj;
mod objects;
mod rendering;
mod scene;

fn main() -> std::io::Result<()> {
  // Scene parameters
  let camera = CameraBuilder::default()
    .fov(60.0)
    .camera_pos(glm::zero())
    .look_at(Vec3::new(0.0, 0.0, 1.0))
    .num_samples(5)
    .build()
    .unwrap();

  let mut scene = SceneBuilder::default()
    .height(800 as usize)
    .width(800 as usize)
    .camera(camera)
    .recursion_depth(1)
    .build()
    .unwrap();

  // Assets setup
  let mat_mirror1 = MaterialBuilder::default()
    .color(Color::new(0.05, 0.75, 0.05))
    .diffuse_coeff(0.7)
    .exp(5.0)
    .refl(0.6)
    .build()
    .unwrap();

  let mat_mirror2 = MaterialBuilder::default()
    .color(Color::new(0.0, 0.5, 0.5))
    .diffuse_coeff(0.4)
    .exp(4.0)
    .refl(0.8)
    .build()
    .unwrap();

  let mat_diffuse1 = MaterialBuilder::default()
    .color(Color::new(0.5, 0.0, 0.5))
    .diffuse_coeff(1.0)
    .build()
    .unwrap();

  let mat_diffuse2 = MaterialBuilder::default()
    .color(Color::new(0.2, 0.9, 0.2))
    .diffuse_coeff(1.0)
    .build()
    .unwrap();

  scene.objects.push(Box::new(Sphere {
    radius: 1.0,
    center: Vec3::new(-1.0, 0.0, 4.0),
    material: Material {
      color: Color::new(0.8, 0.0, 0.0),
      diffuse_coeff: 0.7,
      specular_coeff: 0.5,
      exp: 7.0,
      refl: 0.0,
    },
  }));

  scene.objects.push(Box::new(Sphere {
    radius: 1.0,
    center: Vec3::new(1.0, 1.0, 5.0),
    material: Material {
      color: Color::new(0.75, 1.0, 0.0),
      diffuse_coeff: 0.7,
      specular_coeff: 0.7,
      exp: 5.0,
      refl: 0.0,
    },
  }));

  scene.objects.push(Box::new(Sphere {
    radius: 1.0,
    center: Vec3::new(0.0, 2.5, 6.0),
    material: mat_mirror1,
  }));

  scene.objects.push(Box::new(Sphere {
    radius: 0.6,
    center: Vec3::new(-1.5, 2.0, 4.0),
    material: mat_mirror2,
  }));

  scene.objects.push(Box::new(Sphere {
    radius: 20.0,
    center: Vec3::new(1.0, -20.0, 10.0),
    material: mat_diffuse1,
  }));

  let mut obj = ObjBuilder::default()
    .from_obj_file("assets/teapot.obj")
    .translation(Vec3::new(1.0, -0.2, 4.0))
    .scale(Vec3::new(0.4, 0.4, 0.4))
    .material(mat_diffuse2)
    .build()
    .unwrap();

  obj.update_bbox();
  obj.compute_normals();

  scene.objects.push(Box::new(obj));

  scene.lights.push(Light::PointLight(PointLight {
    intensity: 1.0,
    pos: Vec3::new(0.0, 8.0, 4.0),
  }));

  scene.lights.push(Light::DirectionalLight(DirectionalLight {
    intensity: 0.5,
    dir: Vec3::new(-2.0, 0.0, 1.0),
  }));

  scene.lights.push(Light::AmbientLight(AmbientLight { intensity: 0.2 }));

  // Render to image
  scene.render_to_ppm("image.ppm")?;

  Ok(())
}
