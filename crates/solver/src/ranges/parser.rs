use std::collections::BTreeMap;

use thiserror::Error;

use crate::cards::{all_cards, Rank};

use super::{Combo, ComboIndex, WeightedCombo};

#[derive(Debug, Error)]
pub enum RangeError {
    #[error("invalid token: {0}")]
    InvalidToken(String),
    #[error("invalid weight in token: {0}")]
    InvalidWeight(String),
    #[error("combo error: {0}")]
    Combo(String),
}

#[derive(Debug, Clone)]
pub struct RangeSpec {
    pub token: String,
    pub weight: f64,
}

pub fn parse_range(input: &str) -> Result<Vec<RangeSpec>, RangeError> {
    input
        .split(',')
        .map(str::trim)
        .filter(|t| !t.is_empty())
        .map(|token| {
            if let Some((hand, wt)) = token.split_once(':') {
                let weight = wt
                    .parse::<f64>()
                    .map_err(|_| RangeError::InvalidWeight(token.to_string()))?;
                Ok(RangeSpec {
                    token: hand.to_string(),
                    weight,
                })
            } else {
                Ok(RangeSpec {
                    token: token.to_string(),
                    weight: 1.0,
                })
            }
        })
        .collect()
}

fn parse_rank(c: char) -> Option<Rank> {
    match c {
        '2' => Some(Rank::Two),
        '3' => Some(Rank::Three),
        '4' => Some(Rank::Four),
        '5' => Some(Rank::Five),
        '6' => Some(Rank::Six),
        '7' => Some(Rank::Seven),
        '8' => Some(Rank::Eight),
        '9' => Some(Rank::Nine),
        'T' => Some(Rank::Ten),
        'J' => Some(Rank::Jack),
        'Q' => Some(Rank::Queen),
        'K' => Some(Rank::King),
        'A' => Some(Rank::Ace),
        _ => None,
    }
}

pub fn expand_range(specs: &[RangeSpec]) -> Result<ComboIndex, RangeError> {
    let deck = all_cards();
    let mut out: BTreeMap<(usize, usize), f64> = BTreeMap::new();

    for spec in specs {
        let chars: Vec<char> = spec.token.chars().collect();
        if chars.len() < 2 || chars.len() > 3 {
            return Err(RangeError::InvalidToken(spec.token.clone()));
        }
        let r1 =
            parse_rank(chars[0]).ok_or_else(|| RangeError::InvalidToken(spec.token.clone()))?;
        let r2 =
            parse_rank(chars[1]).ok_or_else(|| RangeError::InvalidToken(spec.token.clone()))?;
        let suited_flag = if chars.len() == 3 {
            Some(chars[2])
        } else {
            None
        };

        for (i, c1) in deck.iter().enumerate() {
            for c2 in deck.iter().skip(i + 1) {
                let ranks = (c1.rank(), c2.rank());
                if !((ranks.0 == r1 && ranks.1 == r2) || (ranks.0 == r2 && ranks.1 == r1)) {
                    continue;
                }
                if let Some(flag) = suited_flag {
                    match flag {
                        's' | 'S' if c1.suit() != c2.suit() => continue,
                        'o' | 'O' if c1.suit() == c2.suit() => continue,
                        _ => {}
                    }
                }

                let combo = Combo::new(*c1, *c2).map_err(RangeError::Combo)?;
                out.insert((combo.c1.index(), combo.c2.index()), spec.weight);
            }
        }
    }

    let combos = out
        .into_iter()
        .map(|((a, b), w)| {
            let combo = Combo::new(deck[a], deck[b]).unwrap();
            WeightedCombo { combo, weight: w }
        })
        .collect();

    Ok(ComboIndex { combos })
}
