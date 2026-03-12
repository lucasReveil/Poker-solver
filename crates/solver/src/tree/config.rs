use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetSizing {
    pub flop_bets: Vec<f64>,
    pub turn_bets: Vec<f64>,
    pub river_bets: Vec<f64>,
    pub raises: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionTreeConfig {
    pub max_raises_per_street: usize,
    pub allow_allin: bool,
    pub bet_sizing: BetSizing,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Action {
    Fold,
    Check,
    Call,
    Bet(f64),
    Raise(f64),
    AllIn,
}
