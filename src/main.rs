use geom::Vec3f;
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
  let mut scene = Scene::new(600, 600, 45.0);

  // Scene setup
  scene.objects.push(Sphere {
    radius: 1.0,
    center: Vec3f { x: -1.0, y: 0.0, z: 4.0 },
    material: Material {
      color: Color { x: 210, y: 0, z: 0 },
      diffuse_coeff: 0.6,
      specular_coeff: 0.5,
      exp: 15.0,
    },
  });

  scene.objects.push(Sphere {
    radius: 1.0,
    center: Vec3f { x: 1.0, y: 1.0, z: 5.0 },
    material: Material {
      color: Color { x: 190, y: 255, z: 0 },
      diffuse_coeff: 0.7,
      specular_coeff: 0.2,
      exp: 5.0,
    },
  });

  scene.lights.push(Light::PointLight(PointLight {
    intensity: 1.0,
    pos: Vec3f { x: 0.0, y: -1.0, z: 4.0 },
  }));

  scene.lights.push(Light::DirectionalLight(DirectionalLight {
    intensity: 0.5,
    dir: Vec3f { x: -2.0, y: 0.0, z: 1.0 },
  }));

  scene.lights.push(Light::AmbientLight(AmbientLight { intensity: 0.2 }));

  scene.render_to_ppm("image.ppm")?;

  Ok(())
}
