use compact_poker::{SCard, SHand};

use num_integer::binomial;
use poker_assistant_lookup::LOOKUP;
use rand::{seq::SliceRandom, Rng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use smallvec::{smallvec, SmallVec};

use super::model::{HandVec, PartialHand};

pub struct SimParams<'a> {
    /// Cards owned or ownable by the player.
    ///
    /// This is usually the set of (hole cards) + (stud cards) + (community cards)
    ///
    /// The undrawn field is the sum of (undrawn hole) + (undrawn stud) + (undrawn community).
    pub player: PartialHand,

    /// Deck to sample from.
    ///
    /// This is usually a set of (full deck) - (known cards owned by all players) - (cards in community) - (cards known to be thrown away)
    pub sample_deck: &'a [SCard],
}

pub struct SimResult {
    /// Cards that we randomly picked.
    pub sampled_undrawn: HandVec,

    /// Using those randomly-picked cards, the best hand we could have gotten.
    pub best_hand: SHand,

    /// The absolute score of this hand.
    pub score: u32,
}

impl SimParams<'_> {
    pub fn run_sample(&self, mut rng: impl Rng) -> SimResult {
        // Generate sampled player cards
        let sampled_undrawn = self
            .sample_deck
            .choose_multiple(&mut rng, self.player.undrawn.into())
            .copied()
            .collect::<HandVec>();

        // Build set of all hands we own
        let mut cards = sampled_undrawn.clone();
        cards.extend(self.player.drawn.iter().copied());

        let (best_hand, score) = score_superhand(&cards[..]);

        SimResult {
            sampled_undrawn,
            best_hand,
            score,
        }
    }

    pub fn n_possibilities(&self) -> u64 {
        binomial(self.sample_deck.len() as u64, self.player.undrawn as u64)
    }

    pub fn run_exhaustive(&self) -> Vec<SimResult> {
        if self.player.undrawn == 0 {
            let (best_hand, score) = score_superhand(&self.player.drawn);

            return vec![SimResult {
                sampled_undrawn: smallvec![],
                best_hand,
                score,
            }];
        }

        let draw_combos = combos(self.sample_deck, self.player.undrawn as usize);

        draw_combos
            .into_par_iter()
            .map(|sampled_undrawn| {
                let mut cards = sampled_undrawn.clone();
                cards.extend(self.player.drawn.iter().copied());
                let (best_hand, score) = score_superhand(&cards[..]);

                SimResult {
                    sampled_undrawn: sampled_undrawn.clone(),
                    best_hand,
                    score,
                }
            })
            .collect()
    }
}

/// Panics if provided hand is empty. Returns (hand of 5, score)
pub fn score_superhand(hand: &[SCard]) -> (SHand, u32) {
    let possible_hands = combos(hand, 5);

    possible_hands
        .into_iter()
        .map(|h| {
            let sh = SHand::from(&h[..]);
            let score = LOOKUP[sh];
            (sh, score)
        })
        .max_by_key(|(_h, s)| *s)
        .unwrap()
}

/// 21 is used because 7 choose 5 = 21.
type CombosVec<T> = SmallVec<[T; 21]>;

/// Enumerate all combinations of the provided slice.
///
/// This only works for 1 <= choose <= 7. However, it should be very efficient.
fn combos<T: Clone>(items: &[T], choose: usize) -> CombosVec<HandVec<T>> {
    if choose == items.len() {
        return smallvec![items.into()];
    }

    let mut current = smallvec![];
    let mut dst = smallvec![];

    macro_rules! make_for_loop {
        ($card_range:expr, ) => {
            dst.push(current.clone());
        };

        // using unary encoding to count nesting
        ($card_range:expr, $_:ident $($tail:ident)*) => {
            let range: &[T] = $card_range;
            for i in 0..range.len() {
                current.push(range[i].clone());
                make_for_loop!(&range[i+1..], $( $tail )*);
                current.pop();
            }
        };
    }

    match choose {
        0 => (),
        1 => {
            make_for_loop!(&items[..], a);
        }
        2 => {
            make_for_loop!(&items[..], a a);
        }
        3 => {
            make_for_loop!(&items[..], a a a);
        }
        4 => {
            make_for_loop!(&items[..], a a a a);
        }
        5 => {
            make_for_loop!(&items[..], a a a a a);
        }
        6 => {
            make_for_loop!(&items[..], a a a a a a);
        }
        7 => {
            make_for_loop!(&items[..], a a a a a a a);
        }
        8 => {
            make_for_loop!(&items[..], a a a a a a a a);
        }
        9 => {
            make_for_loop!(&items[..], a a a a a a a a a);
        }
        10 => {
            make_for_loop!(&items[..], a a a a a a a a a a);
        }
        _ => unimplemented!(),
    }

    dst
}

#[cfg(test)]
mod tests {
    use super::*;
    use smallvec::smallvec;

    #[test]
    fn get_possible_hands_works_equal_size() {
        let result = combos(&[1, 2], 2);

        let expected: Vec<HandVec<i32>> = vec![smallvec![1, 2]];

        assert_eq!(result.to_vec(), expected);
    }

    #[test]
    fn get_possible_hands_works() {
        let result = combos(&[10, 11, 12, 13, 14], 3);

        let expected: Vec<HandVec<i32>> = vec![
            smallvec![10, 11, 12],
            smallvec![10, 11, 13],
            smallvec![10, 11, 14],
            smallvec![10, 12, 13],
            smallvec![10, 12, 14],
            smallvec![10, 13, 14],
            smallvec![11, 12, 13],
            smallvec![11, 12, 14],
            smallvec![11, 13, 14],
            smallvec![12, 13, 14],
        ];

        assert_eq!(result.to_vec(), expected);
    }
}
