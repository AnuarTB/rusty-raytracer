use std::fmt;

#[derive(Debug, Default, Copy, Clone)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

impl Color {
  pub fn new() -> Color {
    Color { r: 0, g: 0, b: 0 }
  }
}

impl fmt::Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} {} {}", self.r, self.g, self.b)
  }
}
