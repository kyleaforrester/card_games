use crate::card::Card;
use crate::card::Suit;

use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Sword {
    pub cards: Vec::<Card>,
    pub trophies: u32,
    pub meditations_remaining: u32,
    pub stumbles: bool,
    pub name: String,
    pub is_human: bool,
}

impl Sword {
    pub fn new(name: &str, is_human: bool) -> Sword {
        Sword {
            cards: Vec::<Card>::new(),
            trophies: 0,
            meditations_remaining: 0,
            stumbles: false,
            name: name.to_string(),
            is_human: is_human,
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn cmp_balance(&self, other: &Sword) -> Ordering {
        let self_sum: u32 = self.cards.iter().filter(|c| c.suit == Suit::Balance).map(|c| c.value).sum();
        let other_sum: u32 = other.cards.iter().filter(|c| c.suit == Suit::Balance).map(|c| c.value).sum();

        let self_max: u32 = match self.cards.iter().filter(|c| c.suit == Suit::Balance).map(|c| c.value).max() {
            Some(v) => v,
            None => 0,
        };
        let other_max: u32 = match self.cards.iter().filter(|c| c.suit == Suit::Balance).map(|c| c.value).max() {
            Some(v) => v,
            None => 0,
        };

        match self_sum.cmp(&other_sum) {
            Ordering::Less => {
                Ordering::Less
            },
            Ordering::Greater => {
                Ordering::Greater
            },
            Ordering::Equal => {
                self_max.cmp(&other_max)
            },
        }
    }
}


