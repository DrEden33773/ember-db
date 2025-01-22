use ember_db::repl;
use std::env;

fn main() {
  let args = env::args().collect::<Vec<_>>();
  repl::main(args);
}
