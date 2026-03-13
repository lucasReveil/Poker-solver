use poker_solver::{
    game::{PotState, RakeConfig, SolverGame, Street},
    ranges::{expand_range, parse_range},
    solver::{solve, SolveInput},
    tree::{compile_tree, ActionTreeConfig, BetSizing},
};

#[test]
fn deterministic_solver_smoke() {
    let board = "AhKdQsJc2c".parse().unwrap();
    let oop = expand_range(&parse_range("AA,AKs").unwrap())
        .unwrap()
        .filter_board(&board);
    let ip = expand_range(&parse_range("KK,KQs").unwrap())
        .unwrap()
        .filter_board(&board);

    let tree = compile_tree(
        &ActionTreeConfig {
            max_raises_per_street: 1,
            allow_allin: true,
            bet_sizing: BetSizing {
                flop_bets: vec![0.5],
                turn_bets: vec![0.75],
                river_bets: vec![1.0],
                raises: vec![2.0],
            },
        },
        Street::River,
    )
    .unwrap();

    let input = SolveInput {
        game: SolverGame {
            initial: PotState {
                pot: 100.0,
                to_call: 0.0,
                stack_oop: 100.0,
                stack_ip: 100.0,
            },
            rake: RakeConfig::default(),
        },
        board,
        oop_range: oop,
        ip_range: ip,
        tree,
        iterations: 50,
    };

    let a = solve(input.clone());
    let b = solve(input);

    assert_eq!(
        a.stats.last().unwrap().root_ev_oop,
        b.stats.last().unwrap().root_ev_oop
    );
    assert_eq!(a.tables.regrets, b.tables.regrets);
}
