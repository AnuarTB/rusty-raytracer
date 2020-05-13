use std::io::prelude::*;
use std::fs::File;
use std::fmt;

mod geom;

#[derive(Default, Clone)]
pub struct Color {
  r: u8,
  g: u8,
  b: u8
}

impl Color {
  fn new(r: u8, g: u8, b: u8) -> Color {
    Color { r, g, b }
  }
}

impl fmt::Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} {} {}", self.r, self.g, self.b)
  }
}

fn main() -> std::io::Result<()> {
  const WIDTH: usize = 800;
  const HEIGHT: usize = 600;
  let mut file = File::create("hello.ppm")?;
  file.write(b"P3\n")?;
  file.write(format!("{} {}\n", &WIDTH, &HEIGHT).as_bytes())?;
  file.write(b"255\n")?;
  let mat = vec![vec![Color::new(0, 0, 0); WIDTH]; HEIGHT];
  for i in 0..HEIGHT {
    for j in 0..WIDTH {
      file.write(format!("{}\t", mat[i][j]).as_bytes())?;
    }
    file.write(b"\n")?;
  }
  Ok(())
}