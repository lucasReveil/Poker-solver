use super::{Card, Rank};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HandRank(pub u32);

fn encode(category: u32, kickers: [u8; 5]) -> HandRank {
    HandRank(
        (category << 20)
            | ((kickers[0] as u32) << 16)
            | ((kickers[1] as u32) << 12)
            | ((kickers[2] as u32) << 8)
            | ((kickers[3] as u32) << 4)
            | (kickers[4] as u32),
    )
}

fn evaluate_5(cards: &[Card; 5]) -> HandRank {
    let mut rank_counts = [0u8; 13];
    let mut suit_counts = [0u8; 4];
    for c in cards {
        rank_counts[c.rank() as usize] += 1;
        suit_counts[c.suit() as usize] += 1;
    }

    let is_flush = suit_counts.iter().any(|&c| c == 5);

    let mut bits = 0u16;
    for (r, &cnt) in rank_counts.iter().enumerate() {
        if cnt > 0 {
            bits |= 1u16 << r;
        }
    }

    let mut straight_high = None;
    for high in (4..13).rev() {
        let window = 0b1_1111u16 << (high - 4);
        if bits & window == window {
            straight_high = Some(high as u8);
            break;
        }
    }
    if straight_high.is_none() {
        let wheel = (1u16 << Rank::Ace as u16)
            | (1u16 << Rank::Two as u16)
            | (1u16 << Rank::Three as u16)
            | (1u16 << Rank::Four as u16)
            | (1u16 << Rank::Five as u16);
        if bits & wheel == wheel {
            straight_high = Some(Rank::Five as u8);
        }
    }

    let mut groups: Vec<(u8, u8)> = rank_counts
        .iter()
        .enumerate()
        .filter(|(_, c)| **c > 0)
        .map(|(r, c)| (*c, r as u8))
        .collect();
    groups.sort_by(|a, b| b.cmp(a));

    if is_flush && straight_high.is_some() {
        return encode(8, [straight_high.unwrap(), 0, 0, 0, 0]);
    }

    if groups[0].0 == 4 {
        return encode(7, [groups[0].1, groups[1].1, 0, 0, 0]);
    }

    if groups[0].0 == 3 && groups[1].0 == 2 {
        return encode(6, [groups[0].1, groups[1].1, 0, 0, 0]);
    }

    if is_flush {
        let mut ranks: Vec<u8> = groups.iter().map(|(_, r)| *r).collect();
        ranks.sort_by(|a, b| b.cmp(a));
        return encode(5, [ranks[0], ranks[1], ranks[2], ranks[3], ranks[4]]);
    }

    if let Some(high) = straight_high {
        return encode(4, [high, 0, 0, 0, 0]);
    }

    if groups[0].0 == 3 {
        return encode(3, [groups[0].1, groups[1].1, groups[2].1, 0, 0]);
    }

    if groups[0].0 == 2 && groups[1].0 == 2 {
        let hi = groups[0].1.max(groups[1].1);
        let lo = groups[0].1.min(groups[1].1);
        return encode(2, [hi, lo, groups[2].1, 0, 0]);
    }

    if groups[0].0 == 2 {
        return encode(1, [groups[0].1, groups[1].1, groups[2].1, groups[3].1, 0]);
    }

    let mut ranks: Vec<u8> = groups.iter().map(|(_, r)| *r).collect();
    ranks.sort_by(|a, b| b.cmp(a));
    encode(0, [ranks[0], ranks[1], ranks[2], ranks[3], ranks[4]])
}

pub fn evaluate_7(cards: &[Card; 7]) -> HandRank {
    let mut best = HandRank(0);
    for a in 0..3 {
        for b in (a + 1)..4 {
            for c in (b + 1)..5 {
                for d in (c + 1)..6 {
                    for e in (d + 1)..7 {
                        let five = [cards[a], cards[b], cards[c], cards[d], cards[e]];
                        let rank = evaluate_5(&five);
                        if rank > best {
                            best = rank;
                        }
                    }
                }
            }
        }
    }
    best
}
