/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.

use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Suite {
    Spades,
    Diamonds,
    Hearts,
    Clubs
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Copy, Clone, Hash)]
enum Rank {
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Copy, Clone, Ord)]
enum ComboType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
struct Combo {
    combo_type: ComboType,
    ranks: Vec<Rank>
}

#[derive(Debug, PartialEq, Eq)]
struct Card {
    rank: Rank,
    suite: Suite,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand<'a> {
    input: &'a str,
    cards: Vec<Card>,
    combo: Combo
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

impl Ord for Combo {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.combo_type != other.combo_type {
            self.combo_type.cmp(&other.combo_type)
        }
        else {
            self.ranks.iter().zip(other.ranks.iter())
                .fold(Ordering::Equal, |acc, (r1, r2)|
                    if acc != Ordering::Equal { acc } // if we're not equal, we're done
                    else { r1.cmp(&r2) }              // otherwise, check the current step
                )
        }
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
        let rank = Rank::new(&input[..len - 1]);
        let suite = Suite::new(&input[len - 1..len]);

        (rank, suite)
    }

    fn is_card_format_valid(card: &str) -> bool {
        let len = card.len();
        len >= 2 &&
        len <= 3 &&
        card.is_ascii()
    }
}

struct RankGroupMap<'a> {
    map: HashMap<&'a str, Vec<Rank>>
}

type RankFreq<'a> = (&'a Rank, &'a u8);

impl <'a>RankGroupMap<'a> {
    fn new() -> Self {
        RankGroupMap {
            map: HashMap::new()
        }
    }

    fn insert(&mut self, key: &'a str, rank: &Rank) {
        let ranks = self.map.entry(key).or_insert(vec![]);
        ranks.push(*rank);
        ranks.sort();
        ranks.reverse();
    }

    fn get(&self, key: &'a str) -> Option<&Vec<Rank>> {
        self.map.get(key)
    }
}

impl Combo {
    fn to_ranks(cards: &Vec<Card>) -> Vec<Rank> {
        let mut ranks: Vec<Rank> = cards.iter()
            .map(|c| c.rank)
            .collect();
        ranks.sort();
        ranks
    }

    fn count_each_rank(mut acc: HashMap<Rank, u8>, rank: &Rank) -> HashMap<Rank, u8> {
        let count = acc.entry(*rank).or_insert(0);
        *count += 1;
        acc
    }

    fn group_ranks<'a>(mut acc: RankGroupMap<'a>, entry: RankFreq) -> RankGroupMap<'a> {
        let (rank, count) = entry;
        let key = match count {
            4 => Some("quadruplet"),
            3 => Some("triplet"),
            2 => Some("pairs"),
            1 => Some("singles"),
            _ => None,
        };

        if let Some(key) = key {
            acc.insert(key, &rank);
        };

        acc
    }

    fn enumerate(ranks: &Vec<Rank>) -> RankGroupMap {
        ranks
            .iter()
            .fold(HashMap::new(), Combo::count_each_rank)
            .iter()
            .fold(RankGroupMap::new(), Combo::group_ranks)
    }

    fn get_straight(g: &RankGroupMap) -> Option<Combo> {
        let ranks = g.get("singles");
        if let None = ranks {
            return None;
        }

        let ranks = ranks.unwrap();
        if ranks == &vec![Rank::Ace, Rank::Five, Rank::Four, Rank::Three, Rank::Two] {
            Some(Combo {
                combo_type: ComboType::Straight,
                ranks: vec![Rank::Five, Rank::Four, Rank::Three, Rank::Two, Rank::Ace]
            })
        }
        else {
            let len = ranks.len();
            for (i, rank) in ranks.iter().enumerate() {
                if i == len - 1 { break; }

                let val1 = *rank as i32;
                let val2 = ranks[i + 1] as i32;
                if (val1 - val2).abs() != 1 {
                    return None;
                }
            }

            Some(Combo {
                combo_type: ComboType::Straight,
                ranks: vec![ranks[0], ranks[1], ranks[2], ranks[3], ranks[4]]
            })
        }
    }

    fn are_cards_of_same_suite(cards: &Vec<Card>) -> bool {
        let suite = cards[0].suite;
        for card in cards.iter() {
            if suite != card.suite {
                return false;
            }
        }
        true
    }

    fn get_full_house(g: &RankGroupMap) -> Option<Combo> {
        if let (Some(t), Some(p)) = (g.get("triplet"), g.get("pairs")) {
            return Some(Combo {
                combo_type: ComboType::FullHouse,
                ranks: vec![t[0], p[0]]
            })
        }

        None
    }

    fn get_four_of_a_kind(g: &RankGroupMap) -> Option<Combo> {
        if let (Some(q), Some(s)) = (g.get("quadruplet"), g.get("singles")) {
            return Some(Combo {
                combo_type: ComboType::FourOfAKind,
                ranks: vec![q[0], s[0]]
            })
        }

        None
    }

    fn get_three_of_a_kind(g: &RankGroupMap) -> Option<Combo> {
        if let (Some(t), Some(s)) = (g.get("triplet"), g.get("singles")) {
            return Some(Combo {
                combo_type: ComboType::ThreeOfAKind,
                ranks: vec![t[0], s[0], s[1]]
            })
        }

        None
    }

    fn get_one_or_two_pairs(g: &RankGroupMap) -> Option<Combo> {
        if let (Some(p), Some(s)) = (g.get("pairs"), g.get("singles")) {
            return match p.len() {
                2 => Some(Combo { combo_type: ComboType::TwoPair, ranks: vec![p[0], p[1], s[0]] }),
                1 => Some(Combo { combo_type: ComboType::OnePair, ranks: vec![p[0], s[0], s[1], s[2]] }),
                _ => None
            }
        }

        None
    }

    fn get_straight_flush(g: &RankGroupMap, c: &Vec<Card>) -> Option<Combo> {
        if let Some(s) = g.get("singles") {
            if Combo::are_cards_of_same_suite(&c) && Combo::get_straight(g).is_some() {
                return Some(Combo {
                    combo_type: ComboType::StraightFlush,
                    ranks: s.clone()
                })
            }
        }

        None
    }

    fn get_flush(g: &RankGroupMap, c: &Vec<Card>) -> Option<Combo> {
        if let Some(s) = g.get("singles") {
            if Combo::are_cards_of_same_suite(&c) {
                return Some(Combo {
                    combo_type: ComboType::Flush,
                    ranks: s.clone()
                })
            }
        }

        None
    }

    fn get_high_card(g: &RankGroupMap) -> Option<Combo> {
        if let Some(s) = g.get("singles") {
            return Some(Combo {
                combo_type: ComboType::HighCard,
                ranks: s.clone()
            })
        }

        None
    }


    fn new(cards: &Vec<Card>) -> Self {
        let ranks = Combo::to_ranks(cards);
        let rank_groups = Combo::enumerate(&ranks);

        if      let Some(c) = Combo::get_full_house(&rank_groups) { c }
        else if let Some(c) = Combo::get_four_of_a_kind(&rank_groups) { c }
        else if let Some(c) = Combo::get_three_of_a_kind(&rank_groups) { c }
        else if let Some(c) = Combo::get_one_or_two_pairs(&rank_groups) { c }
        else if let Some(c) = Combo::get_straight_flush(&rank_groups, cards) { c }
        else if let Some(c) = Combo::get_straight(&rank_groups) { c }
        else if let Some(c) = Combo::get_flush(&rank_groups, cards) { c }
        else if let Some(c) = Combo::get_high_card(&rank_groups) { c }
        else {
            panic!("must... have... combo...");
        }
    }
}

impl <'a>Hand<'a> {
    fn new(input: &'a str) -> Option<Self> {
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

pub fn winning_hands<'a>(input: &[&'a str]) -> Vec<&'a str> {
    let mut hands: Vec<Hand> = input.iter()
                            .map(|h| Hand::new(h))
                            .filter(|o| o.is_some())
                            .map(|o| o.unwrap())
                            .collect();

    if hands.len() != input.len() {
        panic!("Invalid hand found.");
    }

    hands.sort_by(|a, b| a.partial_cmp(b).unwrap());
    hands.reverse();
    prune_losers(hands)
        .iter()
        .map(|h| h.input)
        .collect()
}

fn prune_losers<'a>(hands: Vec<Hand<'a>>) -> Vec<Hand<'a>> {
    if hands.len() == 0 { return hands; }

    let winning_combo = hands[0].combo.clone();
    hands.into_iter()
        .filter(|h| h.combo == winning_combo)
        .collect()
}
