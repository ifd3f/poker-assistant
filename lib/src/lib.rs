use compact_poker::SCard;
use itertools::Itertools;
use poker::{Rank, Suit};
pub use poker_assistant_lookup::HandLookup;
use poker_assistant_lookup::N_HANDS;
use prediction::{model::Player, montecarlo::SimParams};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use smallvec::smallvec;

use crate::prediction::montecarlo::score_hand;

mod prediction;

/*
fn main() {
    let player = Player {
        hole: 2,
        exchanged: 0,
        stud: smallvec![],
    };
    let my_cards = vec![
        SCard::new(Rank::Seven, Suit::Hearts),
        SCard::new(Rank::Queen, Suit::Diamonds),
    ];
    let community = vec![
        SCard::new(Rank::Two, Suit::Hearts),
        SCard::new(Rank::Two, Suit::Diamonds),
        SCard::new(Rank::Eight, Suit::Clubs),
        SCard::new(Rank::King, Suit::Spades),
        SCard::new(Rank::Queen, Suit::Hearts),
    ];
    let n_iters = 10000usize;
    let mut sample_deck = SCard::deck().collect_vec();
    sample_deck.retain(|c| !community.contains(c) && !my_cards.contains(c));

    let params = SimParams {
        player: &player,
        sample_deck: &sample_deck,
        community: &community,
    };

    let _lookup = HandLookup::new();
    let results = (0..n_iters)
        .into_par_iter()
        .map(|_| params.run(rand::thread_rng()))
        .collect::<Vec<_>>();

    for r in results.iter() {
        println!("{}", r.score as f32 / N_HANDS as f32);
    }

    let my_pool = my_cards
        .iter()
        .chain(community.iter())
        .copied()
        .collect_vec();
    let my_score = score_hand(&my_pool).1;
    eprintln!("Your score: {}", my_score as f32 / N_HANDS as f32);

    eprintln!(
        "Likelihood of winning: {}",
        results.iter().filter(|r| r.score < my_score).count() as f32 / n_iters as f32
    );
}
*/
