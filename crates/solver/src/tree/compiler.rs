use crate::game::{Player, Street};

use super::{Action, ActionTreeConfig};

#[derive(Debug, Clone)]
pub enum NodeKind {
    Action { player: Player },
    Chance { street: Street },
    TerminalFold { winner: Player },
    TerminalShowdown,
}

#[derive(Debug, Clone)]
pub struct CompiledNode {
    pub kind: NodeKind,
    pub actions: Vec<Action>,
    pub children: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct CompiledTree {
    pub nodes: Vec<CompiledNode>,
    pub root: usize,
}

impl CompiledTree {
    pub fn validate(&self) -> Result<(), String> {
        for (idx, node) in self.nodes.iter().enumerate() {
            if node.actions.len() != node.children.len() {
                return Err(format!("node {idx} actions/children mismatch"));
            }
            for &c in &node.children {
                if c >= self.nodes.len() {
                    return Err(format!("node {idx} has invalid child {c}"));
                }
            }
        }
        Ok(())
    }
}

fn street_bet_size(cfg: &ActionTreeConfig, street: Street) -> Result<f64, String> {
    let sizing = match street {
        Street::Flop => &cfg.bet_sizing.flop_bets,
        Street::Turn => &cfg.bet_sizing.turn_bets,
        Street::River => &cfg.bet_sizing.river_bets,
    };
    sizing
        .first()
        .copied()
        .ok_or_else(|| format!("missing {:?} bet sizing", street))
}

pub fn compile_tree(cfg: &ActionTreeConfig, street: Street) -> Result<CompiledTree, String> {
    if cfg.bet_sizing.flop_bets.iter().any(|s| *s <= 0.0)
        || cfg.bet_sizing.turn_bets.iter().any(|s| *s <= 0.0)
        || cfg.bet_sizing.river_bets.iter().any(|s| *s <= 0.0)
    {
        return Err("bet sizes must be positive".to_string());
    }

    let mut nodes = Vec::new();

    nodes.push(CompiledNode {
        kind: NodeKind::Action {
            player: Player::Oop,
        },
        actions: vec![Action::Check, Action::Bet(street_bet_size(cfg, street)?)],
        children: vec![1, 2],
    });

    nodes.push(CompiledNode {
        kind: NodeKind::Action { player: Player::Ip },
        actions: vec![Action::Check, Action::Bet(street_bet_size(cfg, street)?)],
        children: vec![3, 4],
    });

    nodes.push(CompiledNode {
        kind: NodeKind::Action { player: Player::Ip },
        actions: vec![Action::Fold, Action::Call],
        children: vec![5, 3],
    });

    let terminal_child = match street.next() {
        Some(next_street) => {
            let chance_idx = 3;
            nodes.push(CompiledNode {
                kind: NodeKind::Chance {
                    street: next_street,
                },
                actions: vec![],
                children: vec![],
            });
            chance_idx
        }
        None => {
            let showdown_idx = 3;
            nodes.push(CompiledNode {
                kind: NodeKind::TerminalShowdown,
                actions: vec![],
                children: vec![],
            });
            showdown_idx
        }
    };

    nodes.push(CompiledNode {
        kind: NodeKind::Action {
            player: Player::Oop,
        },
        actions: vec![Action::Fold, Action::Call],
        children: vec![6, terminal_child],
    });

    nodes.push(CompiledNode {
        kind: NodeKind::TerminalFold {
            winner: Player::Oop,
        },
        actions: vec![],
        children: vec![],
    });

    nodes.push(CompiledNode {
        kind: NodeKind::TerminalFold { winner: Player::Ip },
        actions: vec![],
        children: vec![],
    });

    let tree = CompiledTree { nodes, root: 0 };
    tree.validate()?;
    Ok(tree)
}
