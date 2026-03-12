use crate::tree::{CompiledTree, NodeKind};

#[derive(Debug, Clone)]
pub struct SolverTables {
    pub regrets: Vec<f64>,
    pub strategy_sum: Vec<f64>,
    pub offsets: Vec<(usize, usize)>,
}

impl SolverTables {
    pub fn new(tree: &CompiledTree) -> Self {
        let mut offsets = vec![(0usize, 0usize); tree.nodes.len()];
        let mut cursor = 0usize;
        for (idx, n) in tree.nodes.iter().enumerate() {
            let acts = if matches!(n.kind, NodeKind::Action { .. }) {
                n.actions.len()
            } else {
                0
            };
            offsets[idx] = (cursor, acts);
            cursor += acts;
        }
        Self {
            regrets: vec![0.0; cursor],
            strategy_sum: vec![0.0; cursor],
            offsets,
        }
    }

    pub fn range(&self, node: usize) -> std::ops::Range<usize> {
        let (o, n) = self.offsets[node];
        o..(o + n)
    }
}
