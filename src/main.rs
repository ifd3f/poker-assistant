use compact_poker::SCard;
use hand_lookup::HandLookup;
use itertools::Itertools;
use poker::{Rank, Suit};
use poker_assistant_codegen::N_HANDS;
use prediction::{model::Player, montecarlo::SimParams};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use smallvec::smallvec;

mod hand_lookup;
mod prediction;

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
    let mut sample_deck = SCard::deck().collect_vec();
    sample_deck.retain(|c| !community.contains(c) && !my_cards.contains(c));

    let params = SimParams {
        player: &player,
        sample_deck: &sample_deck,
        community: &community,
    };

    let lookup = HandLookup::new();
    let results = (0..10000usize)
        .into_par_iter()
        .map(|_| params.run(rand::thread_rng(), &lookup))
        .collect::<Vec<_>>();

    for r in results {
        println!("{}", r.score as f32 / N_HANDS as f32);
    }
}
