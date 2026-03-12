use poker_solver::{
    game::{PotState, RakeConfig, SolverGame},
    ranges::{expand_range, parse_range},
    solver::{solve, SolveInput, SolveResult},
    tree::{compile_tree, ActionTreeConfig, BetSizing},
};

fn default_tree() -> poker_solver::tree::CompiledTree {
    compile_tree(&ActionTreeConfig {
        max_raises_per_street: 1,
        allow_allin: true,
        bet_sizing: BetSizing {
            flop_bets: vec![0.5],
            turn_bets: vec![0.75],
            river_bets: vec![1.0],
            raises: vec![2.0],
        },
    })
    .unwrap()
}

fn build_input(board: &str, oop_range: &str, ip_range: &str, iterations: usize) -> SolveInput {
    let board = board.parse().unwrap();
    let oop = expand_range(&parse_range(oop_range).unwrap())
        .unwrap()
        .filter_board(&board);
    let ip = expand_range(&parse_range(ip_range).unwrap())
        .unwrap()
        .filter_board(&board);

    SolveInput {
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
        tree: default_tree(),
        iterations,
    }
}

fn has_hand_dependent_strategy(result: &SolveResult) -> bool {
    for (node_idx, meta) in result.tables.layout.iter().enumerate() {
        if meta.actions < 2 || meta.hands < 2 {
            continue;
        }
        let base = result.tables.average_strategy(node_idx, 0);
        for hand in 1..meta.hands {
            let s = result.tables.average_strategy(node_idx, hand);
            if s.iter().zip(base.iter()).any(|(a, b)| (a - b).abs() > 1e-6) {
                return true;
            }
        }
    }
    false
}

#[test]
fn deterministic_solver_smoke() {
    let a = solve(build_input("AhKdQsJc2c", "AA,AKs", "KK,KQs", 60));
    let b = solve(build_input("AhKdQsJc2c", "AA,AKs", "KK,KQs", 60));

    assert_eq!(
        a.stats.last().unwrap().root_ev_oop,
        b.stats.last().unwrap().root_ev_oop
    );
    assert_eq!(a.tables.regrets, b.tables.regrets);
}

#[test]
fn strategy_differs_across_private_hands() {
    let scenarios = [
        ("AhKdQcJs9d", "AA,22", "KK,33"),
        ("As7d4c2h9s", "AA,AKo,AQo", "KK,QQ,AKo"),
        ("KhQd8c5s2d", "AKo,KQo,QJo", "AA,KK,QQ"),
    ];

    let mut found = false;
    for (board, oop, ip) in scenarios {
        let result = solve(build_input(board, oop, ip, 250));
        if has_hand_dependent_strategy(&result) {
            found = true;
            break;
        }
    }

    assert!(
        found,
        "expected at least one scenario with hand-dependent strategy"
    );
}

#[test]
fn reach_probability_mass_is_consistent() {
    let input = build_input("AhKdQsJc2c", "AA,AKs", "KK,KQs", 5);
    let oop_mass_expected: f64 = input.oop_range.combos.iter().map(|c| c.weight).sum();
    let ip_mass_expected: f64 = input.ip_range.combos.iter().map(|c| c.weight).sum();

    let result = solve(input);
    for stat in &result.stats {
        assert!((stat.oop_reach_mass - oop_mass_expected).abs() < 1e-12);
        assert!((stat.ip_reach_mass - ip_mass_expected).abs() < 1e-12);
    }
}
