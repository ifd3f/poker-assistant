use itertools::Itertools;
use poker::{Card, evaluate::static_lookup::evaluate};
use rayon::slice::ParallelSliceMut;

fn main() {
    let mut hands = all_hands();

    // note: evaluation is free :3
    hands.par_sort_by(|a, b| evaluate(a).unwrap().cmp(&evaluate(b).unwrap()));
}

fn all_hands() -> Vec<Vec<Card>> {
    let deck = Card::generate_deck().collect_vec();
    deck.into_iter().combinations(5).collect_vec()
}
