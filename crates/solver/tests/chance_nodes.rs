use poker_solver::{
    cards::Board,
    game::{PotState, RakeConfig, SolverGame, Street},
    ranges::{Combo, ComboIndex, WeightedCombo},
    solver::{solve, SolveInput},
    tree::{compile_tree, ActionTreeConfig, BetSizing, NodeKind},
};

fn test_tree() -> ActionTreeConfig {
    ActionTreeConfig {
        max_raises_per_street: 1,
        allow_allin: true,
        bet_sizing: BetSizing {
            flop_bets: vec![0.5],
            turn_bets: vec![0.75],
            river_bets: vec![1.0],
            raises: vec![2.0],
        },
    }
}

fn test_game() -> SolverGame {
    SolverGame {
        initial: PotState {
            pot: 100.0,
            to_call: 0.0,
            stack_oop: 100.0,
            stack_ip: 100.0,
        },
        rake: RakeConfig::default(),
    }
}

#[test]
fn chance_node_present_for_flop_and_turn_trees() {
    let flop_tree = compile_tree(&test_tree(), Street::Flop).unwrap();
    let turn_tree = compile_tree(&test_tree(), Street::Turn).unwrap();

    assert!(flop_tree.nodes.iter().any(|n| matches!(
        n.kind,
        NodeKind::Chance {
            street: Street::Turn
        }
    )));
    assert!(turn_tree.nodes.iter().any(|n| matches!(
        n.kind,
        NodeKind::Chance {
            street: Street::River
        }
    )));
}

#[test]
fn card_removal_for_future_cards_is_correct() {
    let board: Board = "AhKdQs".parse().unwrap();
    let oop = Combo::new("Ac".parse().unwrap(), "Ad".parse().unwrap()).unwrap();
    let ip = Combo::new("Kh".parse().unwrap(), "Kc".parse().unwrap()).unwrap();

    let remaining = board.remaining_cards_excluding_mask(oop.mask() | ip.mask());

    assert_eq!(remaining.len(), 45);
    assert!(remaining.iter().all(|c| !board.contains(*c)));
    assert!(!remaining.contains(&oop.c1));
    assert!(!remaining.contains(&oop.c2));
    assert!(!remaining.contains(&ip.c1));
    assert!(!remaining.contains(&ip.c2));
}

#[test]
fn flop_traversal_is_deterministic_with_chance_nodes() {
    let board: Board = "AhKdQs".parse().unwrap();
    let oop = ComboIndex::new(vec![WeightedCombo {
        combo: Combo::new("Ac".parse().unwrap(), "Ad".parse().unwrap()).unwrap(),
        weight: 1.0,
    }])
    .filter_board(&board);
    let ip = ComboIndex::new(vec![WeightedCombo {
        combo: Combo::new("Kh".parse().unwrap(), "Kc".parse().unwrap()).unwrap(),
        weight: 1.0,
    }])
    .filter_board(&board);

    let tree = compile_tree(&test_tree(), Street::Flop).unwrap();

    let input = SolveInput {
        game: test_game(),
        board,
        oop_range: oop,
        ip_range: ip,
        tree,
        iterations: 2,
    };

    let a = solve(input.clone());
    let b = solve(input);

    assert_eq!(
        a.stats.last().unwrap().root_ev_oop,
        b.stats.last().unwrap().root_ev_oop
    );
    assert_eq!(a.tables.regrets, b.tables.regrets);
}
