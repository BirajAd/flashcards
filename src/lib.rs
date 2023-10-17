use std::io;

pub fn user_input(prompt: &str) -> String {
  let mut input: String = String::new();

  println!("{}", prompt);
  match io::stdin().read_line(&mut input) {
    Ok(_s) => {},
    Err(e) => panic!("{e}")
  }

  input.trim().to_string()
}