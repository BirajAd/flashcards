use std::io::{self, Write};
use colored::*;

pub fn user_input(prompt: String, color: &str) -> String {
  let mut input: String = String::new();

  if color == "green" {
    println!("\n{}", prompt.green().bold());
  } else if color == "red" {
    println!("\n{}", prompt.red().bold());
  } else if color == "blue" {
    println!("\n{}", prompt.blue().bold());
  }
 

  
  
  match io::stdin().read_line(&mut input) {
    Ok(_s) => {},
    Err(e) => panic!("{e}")
  }

  input.trim().to_string()
}