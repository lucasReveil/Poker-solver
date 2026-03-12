use poker_solver::{
    cards::Board,
    game::{Player, RakeConfig},
    ranges::{expand_range, parse_range},
    solver::terminal_payoff,
    tree::{compile_tree, ActionTreeConfig, BetSizing, NodeKind},
};

#[test]
fn tree_compile_basic() {
    let tree = compile_tree(&ActionTreeConfig {
        max_raises_per_street: 1,
        allow_allin: true,
        bet_sizing: BetSizing {
            flop_bets: vec![0.5],
            turn_bets: vec![0.75],
            river_bets: vec![1.0],
            raises: vec![2.5],
        },
    })
    .unwrap();
    assert!(matches!(tree.nodes[0].kind, NodeKind::Action { .. }));
}

#[test]
fn terminal_fold_payoff_sign() {
    let board: Board = "AhKdQsJcTc".parse().unwrap();
    let o = expand_range(&parse_range("AA").unwrap()).unwrap().combos[0].combo;
    let i = expand_range(&parse_range("KK").unwrap()).unwrap().combos[0].combo;
    let ev = terminal_payoff(
        &board,
        o,
        i,
        100.0,
        Some(Player::Oop),
        RakeConfig::default(),
    );
    assert_eq!(ev, 50.0);
}

#[test]
fn showdown_payoff_deterministic() {
    let board: Board = "AhKdQsJc2c".parse().unwrap();
    let o = expand_range(&parse_range("AA").unwrap()).unwrap().combos[0].combo;
    let i = expand_range(&parse_range("KK").unwrap()).unwrap().combos[0].combo;
    let ev1 = terminal_payoff(&board, o, i, 100.0, None, RakeConfig::default());
    let ev2 = terminal_payoff(&board, o, i, 100.0, None, RakeConfig::default());
    assert_eq!(ev1, ev2);
}
