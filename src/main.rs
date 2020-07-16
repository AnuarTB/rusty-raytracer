extern crate nalgebra_glm as glm;

use glm::Vec3;
use lights::*;
use objects::Sphere;
use rendering::{Color, Material};
use scene::Scene;

mod geom;
mod lights;
mod objects;
mod rendering;
mod scene;

fn main() -> std::io::Result<()> {
  // Initialize variables and constants
  let mut scene = Scene::new(800, 800, 60.0, 1);

  // Scene setup
  scene.objects.push(Sphere {
    radius: 1.0,
    center: Vec3::new(-1.0, 0.0, 4.0),
    material: Material {
      color: Color { x: 210, y: 0, z: 0 },
      diffuse_coeff: 0.7,
      specular_coeff: 0.5,
      exp: 7.0,
      refl: 0.0,
    },
  });

  scene.objects.push(Sphere {
    radius: 1.0,
    center: Vec3::new(1.0, 1.0, 5.0),
    material: Material {
      color: Color { x: 190, y: 255, z: 0 },
      diffuse_coeff: 0.7,
      specular_coeff: 0.7,
      exp: 5.0,
      refl: 0.0,
    },
  });

  scene.objects.push(Sphere {
    radius: 1.0,
    center: Vec3::new(0.0, 2.5, 6.0),
    material: Material {
      color: Color { x: 20, y: 190, z: 20 },
      diffuse_coeff: 0.7,
      specular_coeff: 0.0,
      exp: 5.0,
      refl: 0.6,
    },
  });

  scene.objects.push(Sphere {
    radius: 20.0,
    center: Vec3::new(1.0, -20.0, 10.0),
    material: Material {
      color: Color { x: 125, y: 0, z: 125 },
      diffuse_coeff: 1.0,
      specular_coeff: 0.0,
      exp: 0.0,
      refl: 0.0,
    },
  });

  scene.lights.push(Light::PointLight(PointLight {
    intensity: 1.0,
    pos: Vec3::new(0.0, 8.0, 4.0),
  }));

  scene.lights.push(Light::DirectionalLight(DirectionalLight {
    intensity: 0.5,
    dir: Vec3::new(-2.0, 0.0, 1.0),
  }));

  scene.lights.push(Light::AmbientLight(AmbientLight { intensity: 0.2 }));

  scene.render_to_ppm("image.ppm")?;

  Ok(())
}
