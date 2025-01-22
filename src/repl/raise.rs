use crate::util::format::{Color, print_color, println_color};

#[inline]
pub(super) fn unrecognized_cmd(cmd: &str) {
  print_color("Unrecognized ", Color::Yellow);
  print_color("command ", Color::Cyan);
  print_color(format!("'{}'", cmd), Color::Magenta);
  println_color(".", Color::Cyan);
}

#[inline]
pub(super) fn unrecognized_kwd(s: &str) {
  print_color("Unrecognized ", Color::Yellow);
  print_color("keyword ", Color::Cyan);
  print!("at the start of ");
  print_color(format!("'{}'", s), Color::Magenta);
  println_color(".", Color::Cyan);
}
