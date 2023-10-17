use std::{ error::Error };
use std::env;
use colored::*;
// use rand::{Rng};

mod flashcards;
use colored::Colorize;
use flashcards::{ Card, CardList };
use some_proj::user_input;

fn example(file_name: String) -> Result<CardList, Box<dyn Error>> {
    let mut contents = csv::Reader::from_path(file_name)
        .expect("Something went wrong reading the file");

    let mut flashcard_list = CardList::new();

    for record in contents.deserialize() {
        let card : Card = record?;
        flashcard_list.add(card);
    }
    // flashcardList.print_all_cards();
    Ok(flashcard_list)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // let mut rng = rand::thread_rng();
    if args.len() > 1 {
        let file_name = &args[1];
        let mut all_cards = match example(file_name.to_string()) {
            Ok(cardlist) => cardlist,
            Err(e) => panic!("{}", e),
        };
        all_cards.shuffle();

        let prompt: String = format!("How many vocabs do you want to try out of {}?: ", all_cards.len());
        let mut total: u32 = match user_input(prompt, "blue").parse() {
            Ok(i) => i,
            Err(e) => panic!("{e}"),
        };

        for card in all_cards.test_knowledge() {
            if total == 0 {
                break;
            }
            total -= 1;
            print!("{} ", &card.term.red().bold());
            let right = user_input(String::from("y/n?:"), "green");
            println!("{}\n", &card.meaning);
        }
        
        // let random_number = rng.gen_range(0..all_cards.len());
        // let random_card = match all_cards.at(random_number) {
        //     Ok(c) => c,
        //     Err(e) => panic!("{e}"),
        // };
        // println!("{} -> {}", random_card.term, random_card.meaning);

    }
}
