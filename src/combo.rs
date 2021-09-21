use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;

use crate::hand::{ Rank, Card };

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
    StraightFlush,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct Combo {
    combo_type: ComboType,
    ranks: Vec<Rank>
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
    pub fn new(cards: &Vec<Card>) -> Self {
        let rank_groups = Combo::get_rank_groups(cards);

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

    fn get_rank_groups<'a>(cards: &Vec<Card>) -> RankGroupMap<'a> {
        cards.iter()
            .map(Card::to_rank)
            .fold(HashMap::new(), Combo::keep_count)
            .iter()
            .fold(RankGroupMap::new(), Combo::group_ranks)
    }

    fn keep_count<T: Eq + Hash>(mut acc: HashMap<T, u8>, key: T) -> HashMap<T, u8> {
        let count = acc.entry(key).or_insert(0);
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

    fn get_straight(g: &RankGroupMap) -> Option<Combo> {
        let ranks = g.get("singles");
        if ranks.is_none() {
            return None;
        }

        let ranks = ranks.unwrap();
        if ranks == &vec![Rank::Ace, Rank::Five, Rank::Four, Rank::Three, Rank::Two] {
            return Some(Combo {
                combo_type: ComboType::Straight,
                ranks: vec![Rank::Five, Rank::Four, Rank::Three, Rank::Two, Rank::Ace]
            })
        }

        if Combo::are_ranks_sequential(ranks) {
            return Some(Combo {
                combo_type: ComboType::Straight,
                ranks: ranks.clone()
            })
        }

        None
    }

    fn are_ranks_sequential(ranks: &Vec<Rank>) -> bool {
        let len = ranks.len();
        let vec1 = &ranks[..len - 1];
        let vec2 = &ranks[1..len];      // offset them by one

        vec1.iter().zip(vec2.iter())
            .map(|(r1, r2)| (*r1 as i8, *r2 as i8))
            .fold(true, |acc, (r1, r2)|
                if acc == false { acc }
                else { (r1 - r2).abs() == 1 }
            )
    }

    fn are_cards_of_same_suite(cards: &Vec<Card>) -> bool {
        cards.iter()
            .all(|c| c.suite == cards[0].suite)
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
}