use std::fs::File;
use std::io::{self, Write};
use std::{ error::Error };
use colored::*;

mod flashcards;
use csv::Reader;
use flashcards::{ Card, CardList };

pub fn user_input(prompt: String, color: &str) -> String {
  let mut input: String = String::new();

  if color == "green" {
    print!("{}", prompt.green().bold());
  } else if color == "red" {
    print!("{}", prompt.red().bold());
  } else if color == "blue" {
    print!("{}", prompt.blue().bold());
  }
  io::stdout().flush();
  
  match io::stdin().read_line(&mut input) {
    Ok(_s) => {},
    Err(e) => panic!("{e}")
  }

  input.trim().to_string()
}

fn get_cards(file_name: String) -> Result<CardList, Box<dyn Error>> {
    let mut contents: Reader<File> = csv::Reader::from_path(file_name)
      .expect("Something went wrong reading the file");

    let mut flashcard_list: CardList = CardList::new();

    for record in contents.deserialize() {
      let card : Card = record?;
      flashcard_list.add(card);
    }
    // flashcardList.print_all_cards();
    Ok(flashcard_list)
}

pub fn test_my_knowledge(file_name: &str) {
  let mut all_cards: CardList = match get_cards(file_name.to_string()) {
    Ok(cardlist) => cardlist,
    Err(e) => panic!("{}", e),
  };
  all_cards.shuffle();

  let prompt: String = format!("How many vocabs do you want to try out of {}?: ", all_cards.len());
  let mut total: u32 = match user_input(prompt, "blue").parse() {
    Ok(i) => i,
    Err(e) => panic!("{e}"),
  };
  let count: usize = total as usize;

  let mut for_review: Vec<&Card> = Vec::new();

  for (row, card) in all_cards.test_knowledge().enumerate() {
    if total == 0 {
      break;
    }
    total -= 1;
    print!("\n{}. {} ", row+1, &card.term.red().bold());
    let reveal = user_input(String::from(""), "green");
    if reveal == "" || reveal == "y" {
      println!(" -->  {}", &card.meaning);
    }
    let right = user_input(String::from("got it? y/n: "), "green");
    if right == "n" {
        for_review.push(&card);
    }
  }
  let review_len: usize = for_review.len();

  if review_len == 0 {
    println!("{} {} {}", "\nCongratulations, you got all".green(), {count}, "terms right.\n".green());
  } else {
    print!("You got {} terms wrong out of {}, ", review_len, count);
    println!("Do you want to review those words?");
    let right = user_input(String::from("y/n?:"), "green");
    if right == "y" || right == "" {
      for card in for_review {
        println!("{} => {}", card.term, card.meaning);
      }
    }
  }
}