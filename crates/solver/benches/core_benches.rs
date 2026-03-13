use criterion::{criterion_group, criterion_main, Criterion};
use poker_solver::{
    game::{PotState, RakeConfig, SolverGame, Street},
    ranges::{expand_range, parse_range},
    solver::{solve, terminal_payoff, SolveInput},
    tree::{compile_tree, ActionTreeConfig, BetSizing},
};

fn bench_range_expand(c: &mut Criterion) {
    c.bench_function("range_expand", |b| {
        b.iter(|| {
            let spec =
                parse_range("22,33,44,55,66,77,88,99,TT,JJ,QQ,KK,AA,AKs,AKo,AQs,KQs").unwrap();
            let _ = expand_range(&spec).unwrap();
        })
    });
}

fn bench_cfr_iteration(c: &mut Criterion) {
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

    c.bench_function("cfr_small_per_iteration", |b| {
        b.iter(|| {
            let _ = solve(SolveInput {
                game: SolverGame {
                    initial: PotState {
                        pot: 100.0,
                        to_call: 0.0,
                        stack_oop: 100.0,
                        stack_ip: 100.0,
                    },
                    rake: RakeConfig::default(),
                },
                board: board.clone(),
                oop_range: oop.clone(),
                ip_range: ip.clone(),
                tree: tree.clone(),
                iterations: 1,
            });
        })
    });
}

fn bench_terminal_eval(c: &mut Criterion) {
    let board = "AhKdQsJc2c".parse().unwrap();
    let oop = expand_range(
        &parse_range("22,33,44,55,66,77,88,99,TT,JJ,QQ,KK,AA,AKs,AKo,AQs,KQs,QJs,JTs").unwrap(),
    )
    .unwrap()
    .filter_board(&board);
    let ip = expand_range(
        &parse_range("22,33,44,55,66,77,88,99,TT,JJ,QQ,KK,AA,AKs,AKo,AQs,KQs,QJs,JTs").unwrap(),
    )
    .unwrap()
    .filter_board(&board);
    let rake = RakeConfig::default();

    c.bench_function("terminal_eval_throughput", |b| {
        b.iter(|| {
            let mut ev = 0.0;
            for o in &oop.combos {
                for i in &ip.combos {
                    if o.combo.mask() & i.combo.mask() != 0 {
                        continue;
                    }
                    ev += terminal_payoff(&board, o.combo, i.combo, 100.0, None, rake);
                }
            }
            ev
        })
    });
}

criterion_group!(
    benches,
    bench_range_expand,
    bench_cfr_iteration,
    bench_terminal_eval
);
criterion_main!(benches);
