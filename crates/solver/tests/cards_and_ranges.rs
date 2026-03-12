use poker_solver::{
    cards::{Board, Card},
    ranges::{expand_range, parse_range},
};

#[test]
fn card_roundtrip() {
    let c: Card = "Ah".parse().unwrap();
    assert_eq!(c.to_string(), "Ah");
}

#[test]
fn board_duplicate_rejected() {
    let b = "AhAhKd".parse::<Board>();
    assert!(b.is_err());
}

#[test]
fn parse_and_expand_range() {
    let spec = parse_range("AA,AKs:0.5,AKo").unwrap();
    let idx = expand_range(&spec).unwrap();
    assert!(!idx.combos.is_empty());
}

#[test]
fn board_filter_removes_conflicts() {
    let board: Board = "AhKdQsJcTc".parse().unwrap();
    let spec = parse_range("AA").unwrap();
    let idx = expand_range(&spec).unwrap().filter_board(&board);
    assert_eq!(idx.combos.len(), 3);
}
