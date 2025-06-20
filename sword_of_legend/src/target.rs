use crate::card::Card;

#[derive(Debug, Clone)]
pub struct Target {
    pub cards: Vec<Card>,
    pub is_cut: bool,
}

impl Target {
    pub fn new() -> Target {
        Target {
            cards: Vec::new(),
            is_cut: false,
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn merge_halves(&mut self, mut other: Target) {
        self.cards.append(&mut other.cards);
    }
}
