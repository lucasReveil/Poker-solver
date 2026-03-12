use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Suit {
    Clubs = 0,
    Diamonds = 1,
    Hearts = 2,
    Spades = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Rank {
    Two = 0,
    Three = 1,
    Four = 2,
    Five = 3,
    Six = 4,
    Seven = 5,
    Eight = 6,
    Nine = 7,
    Ten = 8,
    Jack = 9,
    Queen = 10,
    King = 11,
    Ace = 12,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card(u8);

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self((rank as u8) * 4 + (suit as u8))
    }

    pub fn rank(self) -> Rank {
        match self.0 / 4 {
            0 => Rank::Two,
            1 => Rank::Three,
            2 => Rank::Four,
            3 => Rank::Five,
            4 => Rank::Six,
            5 => Rank::Seven,
            6 => Rank::Eight,
            7 => Rank::Nine,
            8 => Rank::Ten,
            9 => Rank::Jack,
            10 => Rank::Queen,
            11 => Rank::King,
            _ => Rank::Ace,
        }
    }

    pub fn suit(self) -> Suit {
        match self.0 % 4 {
            0 => Suit::Clubs,
            1 => Suit::Diamonds,
            2 => Suit::Hearts,
            _ => Suit::Spades,
        }
    }

    pub fn index(self) -> usize {
        self.0 as usize
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = match self.rank() {
            Rank::Two => '2',
            Rank::Three => '3',
            Rank::Four => '4',
            Rank::Five => '5',
            Rank::Six => '6',
            Rank::Seven => '7',
            Rank::Eight => '8',
            Rank::Nine => '9',
            Rank::Ten => 'T',
            Rank::Jack => 'J',
            Rank::Queen => 'Q',
            Rank::King => 'K',
            Rank::Ace => 'A',
        };
        let s = match self.suit() {
            Suit::Clubs => 'c',
            Suit::Diamonds => 'd',
            Suit::Hearts => 'h',
            Suit::Spades => 's',
        };
        write!(f, "{r}{s}")
    }
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let r = chars.next().ok_or_else(|| "missing rank".to_string())?;
        let su = chars.next().ok_or_else(|| "missing suit".to_string())?;
        if chars.next().is_some() {
            return Err("card must be exactly 2 chars".to_string());
        }
        let rank = match r {
            '2' => Rank::Two,
            '3' => Rank::Three,
            '4' => Rank::Four,
            '5' => Rank::Five,
            '6' => Rank::Six,
            '7' => Rank::Seven,
            '8' => Rank::Eight,
            '9' => Rank::Nine,
            'T' | 't' => Rank::Ten,
            'J' | 'j' => Rank::Jack,
            'Q' | 'q' => Rank::Queen,
            'K' | 'k' => Rank::King,
            'A' | 'a' => Rank::Ace,
            _ => return Err(format!("invalid rank: {r}")),
        };
        let suit = match su {
            'c' | 'C' => Suit::Clubs,
            'd' | 'D' => Suit::Diamonds,
            'h' | 'H' => Suit::Hearts,
            's' | 'S' => Suit::Spades,
            _ => return Err(format!("invalid suit: {su}")),
        };
        Ok(Card::new(rank, suit))
    }
}

pub fn all_cards() -> [Card; 52] {
    let mut cards = [Card(0); 52];
    let mut i = 0;
    while i < 52 {
        cards[i] = Card(i as u8);
        i += 1;
    }
    cards
}
