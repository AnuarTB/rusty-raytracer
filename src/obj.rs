use glm::{make_vec3, U32Vec3, Vec3};

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

#[derive(Debug)]
struct Obj {
  vertices: Vec<Vec3>,
  faces: Vec<U32Vec3>,
}

impl Obj {
  pub fn new() -> Self {
    Obj {
      vertices: Vec::new(),
      faces: Vec::new(),
    }
  }

  pub fn from_obj_file(filename: &str) -> Result<Self, io::Error> {
    let f = File::open(filename)?;
    let f = BufReader::new(f);

    let mut ret_obj = Self::new();

    for line in f.lines() {
      let line = line.unwrap();
      let mut tokens = line.split_whitespace();
      match tokens.next().unwrap() {
        "v" => {
          let vertex: Vec3 = make_vec3(tokens.map(|x| x.parse::<f32>().unwrap()).collect::<Vec<f32>>().as_slice());

          ret_obj.vertices.push(vertex);
        }
        "f" => {
          let face: U32Vec3 = make_vec3(
            tokens
              .map(|x| x.parse::<u32>().unwrap() - 1) // Indentation in faces starts with 1
              .collect::<Vec<u32>>()
              .as_slice(),
          );

          ret_obj.faces.push(face);
        }
        _ => println!("Unexpected token"),
      }
    }

    Ok(ret_obj)
  }
}

mod tests {
  use super::*;
  use glm::{equal, vec3};

  #[test]
  fn print_obj() {
    let obj = Obj::from_obj_file("assets/triangle.obj");
    assert!(obj.is_ok());
    let obj = obj.unwrap();

    assert_eq!(vec3(true, true, true), equal(&obj.vertices[0], &vec3(1.0, 1.0, 0.0)));
    assert_eq!(vec3(true, true, true), equal(&obj.vertices[1], &vec3(1.0, 0.0, 0.0)));
    assert_eq!(vec3(true, true, true), equal(&obj.vertices[2], &vec3(0.0, 1.0, 0.0)));

    assert_eq!(vec3(0, 1, 2), obj.faces[0]);
  }
}
