use glm::{Quat, U32Vec3, Vec3};

use crate::geom::Ray;
use crate::objects::{Hit, Hittable};
use crate::rendering::Material;

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

#[derive(Builder, Debug, Clone)]
pub struct Obj {
  vertices: Vec<Vec3>,
  faces: Vec<U32Vec3>,
  pub material: Material,

  #[builder(default = "glm::zero()")]
  pub translation: Vec3,

  #[builder(default = "glm::zero()")]
  pub scale: Vec3,

  #[builder(default = "glm::zero()")]
  pub rotation: Quat,
}

impl ObjBuilder {
  pub fn from_obj_file(&mut self, filename: &str) -> &mut Self {
    match parse_obj(filename) {
      Ok((vertices, faces)) => {
        self.vertices = Some(vertices);
        self.faces = Some(faces);
        self
      }
      Err(e) => panic!("Can't read from {}, err: {}", filename, e),
    }
  }
}

impl Default for Obj {
  fn default() -> Self {
    Obj {
      vertices: Vec::new(),
      faces: Vec::new(),
      material: Material::new(),
      translation: glm::zero(),
      scale: Vec3::new(1.0, 1.0, 1.0),
      rotation: glm::quat_identity(),
    }
  }
}

impl Obj {
  pub fn from_obj_file(filename: &str) -> Self {
    let mut ret_obj = Self::default();
    match parse_obj(filename) {
      Ok((vertices, faces)) => {
        ret_obj.vertices = vertices;
        ret_obj.faces = faces;
        ret_obj
      }
      Err(e) => panic!("Can't read from {}, err: {}", filename, e),
    }
  }

  pub fn transform_vec(&self, v: Vec3) -> Vec3 {
    let mut transform: glm::Mat4x4 = glm::one();
    transform = glm::translate(&transform, &self.translation);
    transform = glm::scale(&transform, &self.scale);
    let v = glm::quat_rotate_vec3(&self.rotation, &v);
    glm::vec4_to_vec3(&(transform * glm::vec4(v.x, v.y, v.z, 1.0)))
  }
}

mod tests {
  use super::*;
  use glm::{equal, vec3};

  #[test]
  fn read_obj() {
    let obj = Obj::from_obj_file("assets/triangle.obj");

    assert_eq!(vec3(true, true, true), equal(&obj.vertices[0], &vec3(1.0, 1.0, 0.0)));
    assert_eq!(vec3(true, true, true), equal(&obj.vertices[1], &vec3(1.0, 0.0, 0.0)));
    assert_eq!(vec3(true, true, true), equal(&obj.vertices[2], &vec3(0.0, 1.0, 0.0)));

    assert_eq!(vec3(0, 1, 2), obj.faces[0]);
  }
}

fn parse_obj(filename: &str) -> Result<(Vec<Vec3>, Vec<U32Vec3>), io::Error> {
  let f = File::open(filename)?;
  let f = BufReader::new(f);

  let mut faces = Vec::new();
  let mut vertices = Vec::new();

  for line in f.lines() {
    let line = line.unwrap();
    if line.is_empty() {
      continue;
    }
    let mut tokens = line.split_whitespace();
    match tokens.next().unwrap() {
      "v" => {
        let vertex: Vec3 = glm::make_vec3(tokens.map(|x| x.parse::<f32>().unwrap()).collect::<Vec<f32>>().as_slice());

        vertices.push(vertex);
      }
      "f" => {
        let face: U32Vec3 = glm::make_vec3(
          tokens
            .map(|x| x.parse::<u32>().unwrap() - 1) // Indentation in faces starts with 1
            .collect::<Vec<u32>>()
            .as_slice(),
        );

        faces.push(face);
      }
      _ => println!("Unexpected token"),
    }
  }

  Ok((vertices, faces))
}

// @returns: ray's t and normal of triangle
fn hit_triangle(ray: Ray, a: Vec3, b: Vec3, c: Vec3) -> Option<(f32, Vec3)> {
  // 1. Construct a plane from this triangle
  let n = glm::triangle_normal(&a, &b, &c);

  // 2. Intersect ray with the plane
  let t = Vec3::dot(&n, &(a - ray.orig)) / Vec3::dot(&n, &ray.dir);
  if t < 0.0 {
    return None;
  }
  // 3. See if intersection lies on triangle
  let p = ray.at(t);
  let pa = a - p;
  let pb = b - p;
  let pc = c - p;

  // Normalization is done to exclude very short vectors
  let sab = glm::normalize(&Vec3::cross(&pa, &pb));
  let sbc = glm::normalize(&Vec3::cross(&pb, &pc));
  let sca = glm::normalize(&Vec3::cross(&pc, &pa));

  // TODO: Substitute with EPS
  if Vec3::dot(&sab, &sbc) > -1e-4 && Vec3::dot(&sbc, &sca) > 1e-4 {
    Some((t, n))
  } else {
    None
  }
}

impl Hittable for Obj {
  fn hit(&self, ray: Ray) -> Option<Hit> {
    let mut nearest: Option<(f32, Vec3)> = None;

    for face in &self.faces {
      let t = hit_triangle(
        ray,
        self.transform_vec(self.vertices[face.x as usize]),
        self.transform_vec(self.vertices[face.y as usize]),
        self.transform_vec(self.vertices[face.z as usize]),
      );

      if t.is_some() && (nearest.is_none() || nearest.unwrap().0 > t.unwrap().0) {
        nearest = t;
      }
    }

    match nearest {
      None => None,
      Some((t, normal)) => Some(Hit {
        t,
        pos: ray.at(t),
        normal,
        material: &self.material,
      }),
    }
  }
}
