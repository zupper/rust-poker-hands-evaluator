use std::cmp::Ordering;
use std::collections::HashMap;

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
}