use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Player {
    Oop,
    Ip,
}

impl Player {
    pub fn opponent(self) -> Self {
        match self {
            Player::Oop => Player::Ip,
            Player::Ip => Player::Oop,
        }
    }

    pub fn idx(self) -> usize {
        match self {
            Player::Oop => 0,
            Player::Ip => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Street {
    Flop,
    Turn,
    River,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PotState {
    pub pot: f64,
    pub to_call: f64,
    pub stack_oop: f64,
    pub stack_ip: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RakeConfig {
    pub enabled: bool,
    pub cap: f64,
    pub pct: f64,
}

impl Default for RakeConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            cap: 0.0,
            pct: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolverGame {
    pub initial: PotState,
    pub rake: RakeConfig,
}
