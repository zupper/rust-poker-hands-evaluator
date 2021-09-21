/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.

mod hand;
mod combo;

pub use crate::hand::Hand;

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
