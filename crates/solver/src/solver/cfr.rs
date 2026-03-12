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

#[inline]
fn regret_matching_row(regrets: &[f64]) -> Vec<f64> {
    let mut positive_sum = 0.0;
    for r in regrets {
        positive_sum += r.max(0.0);
    }
    if positive_sum <= 1e-12 {
        vec![1.0 / regrets.len() as f64; regrets.len()]
    } else {
        regrets
            .iter()
            .map(|r| r.max(0.0) / positive_sum)
            .collect::<Vec<_>>()
    }
}

/// Counterfactual utilities for traverser hands at a terminal node.
///
/// Returns a vector sized to traverser's hand count where each entry is:
/// `sum_{opp_hand} opp_reach[opp_hand] * U(traverser_hand, opp_hand)`
/// with card-conflicting hand pairs skipped.
fn terminal_values(
    input: &SolveInput,
    winner_if_fold: Option<Player>,
    traverser: Player,
    opp_reach: &[f64],
) -> Vec<f64> {
    match traverser {
        Player::Oop => {
            let mut out = vec![0.0; input.oop_range.combos.len()];
            for (o_idx, o) in input.oop_range.combos.iter().enumerate() {
                let mut v = 0.0;
                for (i_idx, i) in input.ip_range.combos.iter().enumerate() {
                    if o.combo.mask() & i.combo.mask() != 0 {
                        continue;
                    }
                    let u = terminal_payoff(
                        &input.board,
                        o.combo,
                        i.combo,
                        input.game.initial.pot,
                        winner_if_fold,
                        input.game.rake,
                    );
                    v += opp_reach[i_idx] * u;
                }
                out[o_idx] = v;
            }
            out
        }
        Player::Ip => {
            let mut out = vec![0.0; input.ip_range.combos.len()];
            for (i_idx, i) in input.ip_range.combos.iter().enumerate() {
                let mut v = 0.0;
                for (o_idx, o) in input.oop_range.combos.iter().enumerate() {
                    if o.combo.mask() & i.combo.mask() != 0 {
                        continue;
                    }
                    let u_oop = terminal_payoff(
                        &input.board,
                        o.combo,
                        i.combo,
                        input.game.initial.pot,
                        winner_if_fold,
                        input.game.rake,
                    );
                    v += opp_reach[o_idx] * (-u_oop);
                }
                out[i_idx] = v;
            }
            out
        }
    }
}

/// CFR recursion over public tree nodes, with private-hand vectors for the traverser.
///
/// Information-set identity is represented by:
/// - public node index (action history + public board state)
/// - acting player's private hand index within that player's `ComboIndex` ordering.
fn cfr(
    input: &SolveInput,
    tables: &mut SolverTables,
    node: usize,
    traverser: Player,
    traverser_reach: &[f64],
    opp_reach: &[f64],
) -> Vec<f64> {
    let n = &input.tree.nodes[node];

    match n.kind {
        NodeKind::TerminalFold { winner } => {
            terminal_values(input, Some(winner), traverser, opp_reach)
        }
        NodeKind::TerminalShowdown => terminal_values(input, None, traverser, opp_reach),
        NodeKind::Chance { .. } => cfr(
            input,
            tables,
            n.children[0],
            traverser,
            traverser_reach,
            opp_reach,
        ),
        NodeKind::Action { player } if player == traverser => {
            let hand_count = traverser_reach.len();
            let action_count = tables.action_count(node);
            let mut action_values = vec![0.0; hand_count * action_count];

            for a in 0..action_count {
                let mut next_reach = vec![0.0; hand_count];
                for h in 0..hand_count {
                    let row = tables.row_range(node, h);
                    let strategy = regret_matching_row(&tables.regrets[row.clone()]);
                    next_reach[h] = traverser_reach[h] * strategy[a];
                }

                let child_values = cfr(
                    input,
                    tables,
                    n.children[a],
                    traverser,
                    &next_reach,
                    opp_reach,
                );

                for h in 0..hand_count {
                    action_values[h * action_count + a] = child_values[h];
                }
            }

            let mut node_values = vec![0.0; hand_count];
            for h in 0..hand_count {
                let row = tables.row_range(node, h);
                let strategy = regret_matching_row(&tables.regrets[row.clone()]);

                for a in 0..action_count {
                    node_values[h] += strategy[a] * action_values[h * action_count + a];
                }

                for a in 0..action_count {
                    tables.regrets[row.start + a] +=
                        action_values[h * action_count + a] - node_values[h];
                    tables.strategy_sum[row.start + a] += traverser_reach[h] * strategy[a];
                }
            }

            node_values
        }
        NodeKind::Action { .. } => {
            // Opponent node: branch by opponent strategy and mix child counterfactual values.
            let action_count = tables.action_count(node);
            let opp_hand_count = opp_reach.len();
            let mut mixed = vec![0.0; traverser_reach.len()];

            for a in 0..action_count {
                let mut next_opp_reach = vec![0.0; opp_hand_count];
                for oh in 0..opp_hand_count {
                    let row = tables.row_range(node, oh);
                    let strategy = regret_matching_row(&tables.regrets[row.clone()]);
                    next_opp_reach[oh] = opp_reach[oh] * strategy[a];
                }
                let child_values = cfr(
                    input,
                    tables,
                    n.children[a],
                    traverser,
                    traverser_reach,
                    &next_opp_reach,
                );

                for h in 0..mixed.len() {
                    mixed[h] += child_values[h];
                }
            }
            mixed
        }
    }
}

pub fn solve(input: SolveInput) -> SolveResult {
    let mut tables = SolverTables::new(
        &input.tree,
        input.oop_range.combos.len(),
        input.ip_range.combos.len(),
    );
    let mut stats = Vec::with_capacity(input.iterations);

    let init_oop: Vec<f64> = input.oop_range.combos.iter().map(|c| c.weight).collect();
    let init_ip: Vec<f64> = input.ip_range.combos.iter().map(|c| c.weight).collect();

    for i in 1..=input.iterations {
        let oop_values = cfr(
            &input,
            &mut tables,
            input.tree.root,
            Player::Oop,
            &init_oop,
            &init_ip,
        );
        let _ip_values = cfr(
            &input,
            &mut tables,
            input.tree.root,
            Player::Ip,
            &init_ip,
            &init_oop,
        );

        let oop_mass: f64 = init_oop.iter().sum();
        let ip_mass: f64 = init_ip.iter().sum();
        let root_ev_oop = if oop_mass <= 1e-12 {
            0.0
        } else {
            oop_values
                .iter()
                .zip(init_oop.iter())
                .map(|(v, r)| v * r)
                .sum::<f64>()
                / oop_mass
        };

        let avg_abs_regret = if tables.regrets.is_empty() {
            0.0
        } else {
            tables.regrets.iter().map(|r| r.abs()).sum::<f64>() / tables.regrets.len() as f64
        };

        stats.push(IterationStats {
            iteration: i,
            root_ev_oop,
            avg_abs_regret,
            oop_reach_mass: oop_mass,
            ip_reach_mass: ip_mass,
        });
    }

    SolveResult { tables, stats }
}
