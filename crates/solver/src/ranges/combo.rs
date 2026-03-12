use crate::cards::{Board, Card};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Combo {
    pub c1: Card,
    pub c2: Card,
}

impl Combo {
    pub fn new(c1: Card, c2: Card) -> Result<Self, String> {
        if c1 == c2 {
            return Err("combo cannot contain duplicate cards".to_string());
        }
        Ok(if c1.index() < c2.index() {
            Self { c1, c2 }
        } else {
            Self { c1: c2, c2: c1 }
        })
    }

    pub fn conflicts_with_board(&self, board: &Board) -> bool {
        board.contains(self.c1) || board.contains(self.c2)
    }

    pub fn mask(&self) -> u64 {
        (1u64 << self.c1.index()) | (1u64 << self.c2.index())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WeightedCombo {
    pub combo: Combo,
    pub weight: f64,
}

#[derive(Debug, Clone)]
pub struct ComboIndex {
    pub combos: Vec<WeightedCombo>,
}

impl ComboIndex {
    pub fn filter_board(&self, board: &Board) -> Self {
        let combos = self
            .combos
            .iter()
            .copied()
            .filter(|c| !c.combo.conflicts_with_board(board))
            .collect();
        Self { combos }
    }
}
