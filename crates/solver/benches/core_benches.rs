use criterion::{criterion_group, criterion_main, Criterion};
use poker_solver::{
    game::{PotState, RakeConfig, SolverGame, Street},
    ranges::{expand_range, parse_range},
    solver::{solve, SolveInput},
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
    c.bench_function("cfr_small_100_iters", |b| {
        b.iter(|| {
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
                board,
                oop_range: oop,
                ip_range: ip,
                tree,
                iterations: 100,
            });
        })
    });
}

criterion_group!(benches, bench_range_expand, bench_cfr_iteration);
criterion_main!(benches);
