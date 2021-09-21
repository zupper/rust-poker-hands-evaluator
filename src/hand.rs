use std::cmp::Ordering;
use crate::combo::Combo;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Suite {
    Spades,
    Diamonds,
    Hearts,
    Clubs
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Copy, Clone, Hash)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Card {
    pub rank: Rank,
    pub suite: Suite,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand<'a> {
    pub input: &'a str,
    pub cards: Vec<Card>,
    pub combo: Combo
}

impl Suite {
    fn new(input: &str) -> Option<Self> {
        match input {
            "C" => Some(Suite::Clubs),
            "D" => Some(Suite::Diamonds),
            "H" => Some(Suite::Hearts),
            "S" => Some(Suite::Spades),
            _ => None
        }
    }
}

impl Rank {
    fn new(input: &str) -> Option<Self> {
        match input {
            "2" => Some(Rank::Two),
            "3" => Some(Rank::Three),
            "4" => Some(Rank::Four),
            "5" => Some(Rank::Five),
            "6" => Some(Rank::Six),
            "7" => Some(Rank::Seven),
            "8" => Some(Rank::Eight),
            "9" => Some(Rank::Nine),
            "10" => Some(Rank::Ten),
            "J" => Some(Rank::Jack),
            "Q" => Some(Rank::Queen),
            "K" => Some(Rank::King),
            "A" => Some(Rank::Ace),
            _ => None
        }
    }
}

impl Ord for Rank {
    fn cmp(&self, other: &Self) -> Ordering {
        let val1 = *self as u32;
        let val2 = *other as u32;
        (val1).cmp(&val2)
    }
}

impl Card {
    fn new(input: &str) -> Option<Self> {
        match Card::enumerate(input) {
            (Some(rank), Some(suite)) => Some(Card { suite, rank }),
            _ => None
        }
    }

    fn enumerate(input: &str) -> (Option<Rank>, Option<Suite>) {
        if !Card::is_card_format_valid(input) {
            return (None, None);
        }

        let len = input.len();
        (
            Rank::new(&input[..len - 1]),
            Suite::new(&input[len - 1..len]),
        )
    }

    fn is_card_format_valid(card: &str) -> bool {
        let len = card.len();
        len >= 2 &&
        len <= 3 &&
        card.is_ascii()
    }
}

impl <'a>Hand<'a> {
    pub fn new(input: &'a str) -> Option<Self> {
        let cards: Vec<Card> = input
                        .split_whitespace()
                        .map(Card::new)
                        .filter(|c| c.is_some())
                        .map(|o| o.unwrap())
                        .collect();

        if cards.len() != 5 {
            None
        }
        else {
            let combo = Combo::new(&cards);
            Some(Hand {
                input,
                cards,
                combo
            })
        }
    }
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.combo.cmp(&other.combo))
    }
}
