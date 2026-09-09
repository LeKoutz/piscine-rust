use rand::Rng;

#[derive(Debug, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, PartialEq)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    /// returns a random Suit (Heart, Diamond, Spade or Club
    pub fn random() -> Suit {
        Self::translate(rand::thread_rng().gen_range(1..=4))
    }

    /// converts an integer value (u8) to a suit (1 -> Heart, 2 -> Diamonds, 3 -> Spade, 4 -> Club)
    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Self::Heart,
            2 => Self::Diamond,
            3 => Self::Spade,
            4 => Self::Club,
            _ => panic!(),
        }
    }
}

impl Rank {
    ///  returns a random Rank (Ace, King, Queen or Jack, Number 2-10 u8)
    pub fn random() -> Rank {
        Self::translate(rand::thread_rng().gen_range(1..=13))
    }

    /// converts an integer value (u8) to a rank ( 1 -> Ace, 2 -> 2, .., 10 -> 10, 11 -> Jack, 12 -> Queen, 13 -> King)
    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Self::Ace,
            2..=10 => Self::Number(value),
            11 => Self::Jack,
            12 => Self::Queen,
            13 => Self::King,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

/// returns true if the card passed as an argument is an ace of spades
pub fn winner_card(card: &Card) -> bool {
    *card == Card {
        suit: Suit::Spade,
        rank: Rank::Ace,
    }
}


