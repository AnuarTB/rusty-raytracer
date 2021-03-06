use glm::{Quat, U32Vec3, Vec3};

use crate::geom::Ray;
use crate::objects::{Hit, Hittable, AABB};
use crate::rendering::Material;

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::mem::swap;

#[derive(Builder, Debug, Clone)]
pub struct Obj {
  pub vertices: Vec<Vec3>,
  faces: Vec<U32Vec3>,

  #[builder(default = "Vec::new()")]
  normals: Vec<Vec3>,

  pub material: Material,

  #[builder(default = "glm::zero()")]
  pub translation: Vec3,

  #[builder(default = "glm::zero()")]
  pub scale: Vec3,

  #[builder(default = "glm::zero()")]
  pub rotation: Quat,

  #[builder(default = "AABB::default()")]
  pub aabb: AABB,
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
      normals: Vec::new(),
      material: Material::new(),
      translation: glm::zero(),
      scale: Vec3::new(1.0, 1.0, 1.0),
      rotation: glm::quat_identity(),
      aabb: AABB::default(),
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
        ret_obj.update_bbox();
        ret_obj.compute_normals();
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

  pub fn compute_normals(&mut self) {
    for face in &self.faces {
      self.normals.push(glm::triangle_normal(
        &self.vertices[face.x as usize],
        &self.vertices[face.y as usize],
        &self.vertices[face.z as usize],
      ));
    }
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

fn hit_triangle_with_norm(ray: Ray, a: Vec3, b: Vec3, c: Vec3, n: Vec3) -> Option<(f32, Vec3)> {
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

// @returns: ray's t and normal of triangle
fn hit_triangle(ray: Ray, a: Vec3, b: Vec3, c: Vec3) -> Option<(f32, Vec3)> {
  // 1. Construct a plane from this triangle
  let n = glm::triangle_normal(&a, &b, &c);
  hit_triangle_with_norm(ray, a, b, c, n)
}

impl Hittable for Obj {
  fn hit(&self, ray: Ray) -> Option<Hit> {
    let mut nearest: Option<(f32, Vec3)> = None;

    for (i, face) in self.faces.iter().enumerate() {
      let t = hit_triangle_with_norm(
        ray,
        self.transform_vec(self.vertices[face.x as usize]),
        self.transform_vec(self.vertices[face.y as usize]),
        self.transform_vec(self.vertices[face.z as usize]),
        self.normals[i],
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

  fn update_bbox(&mut self) {
    for vertex in &self.vertices {
      self.aabb.mn = glm::min2(&self.aabb.mn, &self.transform_vec(*vertex));
      self.aabb.mx = glm::max2(&self.aabb.mx, &self.transform_vec(*vertex));
    }
  }

  fn aabb(&self, ray: Ray) -> bool {
    aabb_hit(self.aabb, ray)
  }
}

fn aabb_hit(aabb: AABB, ray: Ray) -> bool {
  let mut tmin = (aabb.mn.x - ray.orig.x) / ray.dir.x;
  let mut tmax = (aabb.mx.x - ray.orig.x) / ray.dir.x;

  if tmin > tmax {
    swap(&mut tmin, &mut tmax);
  }

  let mut tymin = (aabb.mn.y - ray.orig.y) / ray.dir.y;
  let mut tymax = (aabb.mx.y - ray.orig.y) / ray.dir.y;

  if tymin > tymax {
    swap(&mut tymin, &mut tymax);
  }

  if tmin > tymax || tymin > tmax {
    return false;
  }

  if tymin > tmin {
    tmin = tymin;
  }

  if tymax < tmax {
    tmax = tymax;
  }

  let mut tzmin = (aabb.mn.z - ray.orig.z) / ray.dir.z;
  let mut tzmax = (aabb.mx.z - ray.orig.z) / ray.dir.z;

  if tzmin > tzmax {
    swap(&mut tzmin, &mut tzmax);
  }

  if tmin > tzmax || tzmin > tmax {
    return false;
  }

  if tzmin > tmin {
    tmin = tymin;
  }

  if tzmax < tmax {
    tmax = tymax;
  }

  true
}
