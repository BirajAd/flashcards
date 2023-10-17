use rand::{seq::SliceRandom, thread_rng};
use serde::Deserialize;
use std::io;

#[derive(Deserialize)]
pub struct Card {
  pub term: String,
  pub meaning: String
}

pub struct CardList {
  pub cards: Vec<Card>
}

// impl Card {
//   pub fn new(term: String, meaning: String) -> Card {
//     Card { term, meaning }
//   }
// }

impl CardList {

  pub fn new() -> CardList {
    CardList { cards: Vec::new() }
  }

  pub fn at(&self, index: usize) -> Result<&Card, io::Error> {
    Ok(&self.cards[index])
  }

  pub fn len(&self) -> usize {
    return self.cards.len();
  }

  pub fn add(&mut self, card: Card) {
    self.cards.push(card);
  }

  pub fn shuffle(&mut self) {
    self.cards.shuffle(&mut thread_rng());
  }

  pub fn test_knowledge(&self) -> std::slice::Iter<'_, Card> {
    self.cards.iter()
  }

  #[allow(dead_code)]
  pub fn print_all_cards(self) {
    let size = self.cards.len();
    println!("Total cards: {size}");
    for c in self.cards {
      println!("{} -> {}", c.term, c.meaning);
    }
  }
}