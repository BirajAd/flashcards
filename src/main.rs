use std::env;
// use rand::{Rng};

use some_proj::{ test_my_knowledge };

fn main() {
  let args: Vec<String> = env::args().collect();
  // let mut rng = rand::thread_rng();
  if args.len() > 1 {
    let file_name = &args[1];
    test_my_knowledge(file_name)
  }
}
