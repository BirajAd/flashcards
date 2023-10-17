use std::{ error::Error };
use std::env;
// use rand::{Rng};

mod flashcards;
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

        let mut total: u32 = match user_input("How many vocabs do you want to try?: ").parse() {
            Ok(i) => i,
            Err(e) => panic!("{e}"),
        };

        for card in all_cards.test_knowledge() {
            if total == 0 {
                break;
            }
            total -= 1;
            println!("{} => {}", card.term, card.meaning);
        }
        
        // let random_number = rng.gen_range(0..all_cards.len());
        // let random_card = match all_cards.at(random_number) {
        //     Ok(c) => c,
        //     Err(e) => panic!("{e}"),
        // };
        // println!("{} -> {}", random_card.term, random_card.meaning);

    }
}
