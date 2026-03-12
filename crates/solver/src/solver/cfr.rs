use crate::{
    cards::Board,
    game::{Player, SolverGame},
    ranges::ComboIndex,
    solver::{stats::IterationStats, tables::SolverTables, utility::terminal_payoff},
    tree::{CompiledTree, NodeKind},
};

#[derive(Debug, Clone)]
pub struct SolveInput {
    pub game: SolverGame,
    pub board: Board,
    pub oop_range: ComboIndex,
    pub ip_range: ComboIndex,
    pub tree: CompiledTree,
    pub iterations: usize,
}

#[derive(Debug, Clone)]
pub struct SolveResult {
    pub tables: SolverTables,
    pub stats: Vec<IterationStats>,
}

fn regret_matching(regrets: &[f64]) -> Vec<f64> {
    let positive: Vec<f64> = regrets.iter().map(|r| r.max(0.0)).collect();
    let s: f64 = positive.iter().sum();
    if s <= 1e-12 {
        vec![1.0 / regrets.len() as f64; regrets.len()]
    } else {
        positive.into_iter().map(|p| p / s).collect()
    }
}

fn terminal_ev(input: &SolveInput, node: usize) -> f64 {
    let n = &input.tree.nodes[node];
    match n.kind {
        NodeKind::TerminalFold { winner } => {
            let mut ev = 0.0;
            let mut w = 0.0;
            for o in &input.oop_range.combos {
                for i in &input.ip_range.combos {
                    if o.combo.mask() & i.combo.mask() != 0 {
                        continue;
                    }
                    let weight = o.weight * i.weight;
                    ev += weight
                        * terminal_payoff(
                            &input.board,
                            o.combo,
                            i.combo,
                            input.game.initial.pot,
                            Some(winner),
                            input.game.rake,
                        );
                    w += weight;
                }
            }
            if w > 0.0 {
                ev / w
            } else {
                0.0
            }
        }
        NodeKind::TerminalShowdown => {
            let mut ev = 0.0;
            let mut w = 0.0;
            for o in &input.oop_range.combos {
                for i in &input.ip_range.combos {
                    if o.combo.mask() & i.combo.mask() != 0 {
                        continue;
                    }
                    let weight = o.weight * i.weight;
                    ev += weight
                        * terminal_payoff(
                            &input.board,
                            o.combo,
                            i.combo,
                            input.game.initial.pot,
                            None,
                            input.game.rake,
                        );
                    w += weight;
                }
            }
            if w > 0.0 {
                ev / w
            } else {
                0.0
            }
        }
        _ => 0.0,
    }
}

fn cfr(
    input: &SolveInput,
    tables: &mut SolverTables,
    node: usize,
    updating: Player,
    p_oop: f64,
    p_ip: f64,
) -> f64 {
    let n = &input.tree.nodes[node];
    match n.kind {
        NodeKind::TerminalFold { .. } | NodeKind::TerminalShowdown => {
            return terminal_ev(input, node)
        }
        NodeKind::Chance { .. } => return cfr(input, tables, n.children[0], updating, p_oop, p_ip),
        NodeKind::Action { player } => {
            let r = tables.range(node);
            let strategy = regret_matching(&tables.regrets[r.clone()]);
            let mut action_utils = vec![0.0; strategy.len()];
            let mut node_util = 0.0;
            for (a, &child) in n.children.iter().enumerate() {
                let util = match player {
                    Player::Oop => cfr(input, tables, child, updating, p_oop * strategy[a], p_ip),
                    Player::Ip => cfr(input, tables, child, updating, p_oop, p_ip * strategy[a]),
                };
                action_utils[a] = util;
                node_util += strategy[a] * util;
            }

            let reach_weight = if player == updating {
                if player == Player::Oop {
                    p_ip
                } else {
                    p_oop
                }
            } else {
                0.0
            };
            for i in 0..strategy.len() {
                if player == updating {
                    tables.regrets[r.start + i] += reach_weight * (action_utils[i] - node_util);
                }
                let self_reach = if player == Player::Oop { p_oop } else { p_ip };
                tables.strategy_sum[r.start + i] += self_reach * strategy[i];
            }

            node_util
        }
    }
}

pub fn solve(input: SolveInput) -> SolveResult {
    let mut tables = SolverTables::new(&input.tree);
    let mut stats = Vec::with_capacity(input.iterations);

    for i in 1..=input.iterations {
        let ev = cfr(&input, &mut tables, input.tree.root, Player::Oop, 1.0, 1.0);
        let _ = cfr(&input, &mut tables, input.tree.root, Player::Ip, 1.0, 1.0);
        let avg_abs_regret = if tables.regrets.is_empty() {
            0.0
        } else {
            tables.regrets.iter().map(|r| r.abs()).sum::<f64>() / tables.regrets.len() as f64
        };
        stats.push(IterationStats {
            iteration: i,
            root_ev_oop: ev,
            avg_abs_regret,
        });
    }

    SolveResult { tables, stats }
}
