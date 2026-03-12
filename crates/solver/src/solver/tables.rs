use crate::{
    game::Player,
    tree::{CompiledTree, NodeKind},
};

/// Per-node metadata for contiguous info-set table layout.
///
/// Memory layout is row-major by hand then action:
/// `cell(node, hand_idx, action_idx) = offset + hand_idx * actions + action_idx`.
///
/// This avoids hash lookups in CFR hot paths and provides deterministic indexing.
#[derive(Debug, Clone, Copy)]
pub struct NodeTableLayout {
    pub offset: usize,
    pub actions: usize,
    pub hands: usize,
    pub player: Option<Player>,
}

#[derive(Debug, Clone)]
pub struct SolverTables {
    /// Regret table for each information set (public node + private hand + action).
    pub regrets: Vec<f64>,
    /// Cumulative strategy table with the same index mapping as `regrets`.
    pub strategy_sum: Vec<f64>,
    pub layout: Vec<NodeTableLayout>,
}

impl SolverTables {
    pub fn new(tree: &CompiledTree, oop_hands: usize, ip_hands: usize) -> Self {
        let mut layout = vec![
            NodeTableLayout {
                offset: 0,
                actions: 0,
                hands: 0,
                player: None,
            };
            tree.nodes.len()
        ];

        let mut cursor = 0usize;
        for (node_idx, node) in tree.nodes.iter().enumerate() {
            if let NodeKind::Action { player } = node.kind {
                let actions = node.actions.len();
                let hands = match player {
                    Player::Oop => oop_hands,
                    Player::Ip => ip_hands,
                };
                layout[node_idx] = NodeTableLayout {
                    offset: cursor,
                    actions,
                    hands,
                    player: Some(player),
                };
                cursor += actions * hands;
            }
        }

        Self {
            regrets: vec![0.0; cursor],
            strategy_sum: vec![0.0; cursor],
            layout,
        }
    }

    #[inline]
    pub fn row_range(&self, node: usize, hand_idx: usize) -> std::ops::Range<usize> {
        let meta = self.layout[node];
        let start = meta.offset + hand_idx * meta.actions;
        start..(start + meta.actions)
    }

    #[inline]
    pub fn action_count(&self, node: usize) -> usize {
        self.layout[node].actions
    }

    pub fn average_strategy(&self, node: usize, hand_idx: usize) -> Vec<f64> {
        let row = self.row_range(node, hand_idx);
        let sum: f64 = self.strategy_sum[row.clone()].iter().sum();
        if sum <= 1e-12 {
            vec![1.0 / row.len() as f64; row.len()]
        } else {
            self.strategy_sum[row].iter().map(|v| v / sum).collect()
        }
    }
}
