#[macro_use]
extern crate dotenv_codegen;

use std::env;

mod utils {
  pub mod request;
}

mod days {
  pub mod day1;
}

fn main() {
  let args: Vec<String> = env::args().collect();

  match args[1].parse::<i32>().expect(&format!("Expected a number, got {}", args[1])) {
    1 => days::day1::run(),
    _ => panic!("Expected day as first argument")
  };
}
