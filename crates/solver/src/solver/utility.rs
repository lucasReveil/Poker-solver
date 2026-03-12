use crate::{
    cards::{evaluate_7, Board},
    game::{Player, RakeConfig},
    ranges::Combo,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShowdownResult {
    OopWin,
    IpWin,
    Tie,
}

pub fn showdown_result(board: &Board, oop: Combo, ip: Combo) -> ShowdownResult {
    assert!(
        board.cards().len() == 5,
        "showdown requires 5-card board in MVP"
    );
    let oop_cards = [
        board.cards()[0],
        board.cards()[1],
        board.cards()[2],
        board.cards()[3],
        board.cards()[4],
        oop.c1,
        oop.c2,
    ];
    let ip_cards = [
        board.cards()[0],
        board.cards()[1],
        board.cards()[2],
        board.cards()[3],
        board.cards()[4],
        ip.c1,
        ip.c2,
    ];
    let o = evaluate_7(&oop_cards);
    let i = evaluate_7(&ip_cards);
    if o > i {
        ShowdownResult::OopWin
    } else if i > o {
        ShowdownResult::IpWin
    } else {
        ShowdownResult::Tie
    }
}

pub fn terminal_payoff(
    board: &Board,
    oop: Combo,
    ip: Combo,
    pot: f64,
    fold_winner: Option<Player>,
    rake: RakeConfig,
) -> f64 {
    if let Some(winner) = fold_winner {
        return if winner == Player::Oop {
            pot / 2.0
        } else {
            -pot / 2.0
        };
    }

    let mut won = match showdown_result(board, oop, ip) {
        ShowdownResult::OopWin => pot / 2.0,
        ShowdownResult::IpWin => -pot / 2.0,
        ShowdownResult::Tie => 0.0,
    };

    if rake.enabled && won != 0.0 {
        let rake_amount = (pot * rake.pct).min(rake.cap);
        won -= rake_amount / 2.0 * won.signum();
    }
    won
}
