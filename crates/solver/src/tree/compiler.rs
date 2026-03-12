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

pub fn compile_tree(cfg: &ActionTreeConfig) -> Result<CompiledTree, String> {
    if cfg.bet_sizing.flop_bets.iter().any(|s| *s <= 0.0)
        || cfg.bet_sizing.turn_bets.iter().any(|s| *s <= 0.0)
        || cfg.bet_sizing.river_bets.iter().any(|s| *s <= 0.0)
    {
        return Err("bet sizes must be positive".to_string());
    }

    // Simplified deterministic MVP tree: one-bet-per-street branch with optional river showdown.
    let mut nodes = Vec::new();

    // 0 root OOP action
    nodes.push(CompiledNode {
        kind: NodeKind::Action {
            player: Player::Oop,
        },
        actions: vec![Action::Check, Action::Bet(cfg.bet_sizing.flop_bets[0])],
        children: vec![1, 2],
    });

    // 1 IP response to check
    nodes.push(CompiledNode {
        kind: NodeKind::Action { player: Player::Ip },
        actions: vec![Action::Check, Action::Bet(cfg.bet_sizing.flop_bets[0])],
        children: vec![3, 4],
    });

    // 2 IP response to OOP bet
    nodes.push(CompiledNode {
        kind: NodeKind::Action { player: Player::Ip },
        actions: vec![Action::Fold, Action::Call],
        children: vec![5, 3],
    });

    // 3 chance to river as terminal simplification for MVP
    nodes.push(CompiledNode {
        kind: NodeKind::TerminalShowdown,
        actions: vec![],
        children: vec![],
    });

    // 4 OOP response to IP bet after check
    nodes.push(CompiledNode {
        kind: NodeKind::Action {
            player: Player::Oop,
        },
        actions: vec![Action::Fold, Action::Call],
        children: vec![6, 3],
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
