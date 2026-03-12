use std::str::FromStr;

use super::{all_cards, Card};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    cards: Vec<Card>,
    mask: u64,
}

impl Board {
    pub fn new(cards: Vec<Card>) -> Result<Self, String> {
        if !(cards.len() == 3 || cards.len() == 4 || cards.len() == 5) {
            return Err("board must have 3, 4, or 5 cards".to_string());
        }
        let mut mask = 0u64;
        for c in &cards {
            let bit = 1u64 << c.index();
            if mask & bit != 0 {
                return Err("board has duplicate card".to_string());
            }
            mask |= bit;
        }
        Ok(Self { cards, mask })
    }

    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    pub fn contains(&self, card: Card) -> bool {
        self.mask & (1u64 << card.index()) != 0
    }

    pub fn mask(&self) -> u64 {
        self.mask
    }

    pub fn with_card(&self, card: Card) -> Result<Self, String> {
        if self.contains(card) {
            return Err("board has duplicate card".to_string());
        }
        let mut cards = self.cards.clone();
        cards.push(card);
        Board::new(cards)
    }

    pub fn remaining_cards_excluding_mask(&self, excluded_mask: u64) -> Vec<Card> {
        let dead = self.mask | excluded_mask;
        all_cards()
            .into_iter()
            .filter(|c| dead & (1u64 << c.index()) == 0)
            .collect()
    }
}

impl FromStr for Board {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cleaned = s.replace(' ', "");
        if cleaned.len() % 2 != 0 {
            return Err("board string must contain 2-char cards".to_string());
        }
        let mut cards = Vec::new();
        let mut i = 0;
        while i < cleaned.len() {
            let c = Card::from_str(&cleaned[i..i + 2])?;
            cards.push(c);
            i += 2;
        }
        Board::new(cards)
    }
}
