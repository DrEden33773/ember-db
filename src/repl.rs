use crate::util::format::{Color, print_color, println_color};
use std::io::{self, Write};

pub mod raise;

#[inline]
fn print_prompt() {
  print_color("EmberDB > ", Color::Blue);
  // Remember to flush the buffer,
  // otherwise it'll display after user inputs.
  io::stdout().flush().unwrap();
}

pub fn main(_args: Vec<String>) {
  loop {
    print_prompt();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim();
    match input {
      "" => continue,
      ".exit" => {
        println_color("GoodBye!", Color::Green);
        break;
      }
      ".clear" => {
        print!("\x1b[2J\x1b[1;1H");
      }
      _ => raise::unrecognized_cmd(input),
    }
  }
}
