use std::fmt::Display;

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Color {
  Reset = 0,
  Red = 31,
  Green = 32,
  Yellow = 33,
  Blue = 34,
  Magenta = 35,
  Cyan = 36,
  White = 37,
}

pub fn format_color(content: impl Display, color: Color) -> String {
  let set = format!("\x1b[{}m", color as u8);
  let reset = "\x1b[0m";
  format!("{}{}{}", set, content, reset)
}

#[inline]
pub fn println_color(content: impl Display, color: Color) {
  println!("{}", format_color(content, color));
}

#[inline]
pub fn print_color(content: impl Display, color: Color) {
  print!("{}", format_color(content, color));
}

#[cfg(test)]
mod tests {
  use super::Color;

  #[test]
  fn test_enum_cast() {
    let enums = [
      Color::Reset,
      Color::Red,
      Color::Green,
      Color::Yellow,
      Color::Blue,
      Color::Magenta,
      Color::Cyan,
      Color::White,
    ];
    let values = [0u8, 31, 32, 33, 34, 35, 36, 37];
    for (&e, &v) in enums.iter().zip(values.iter()) {
      assert_eq!(v, e as u8);
    }
  }
}
