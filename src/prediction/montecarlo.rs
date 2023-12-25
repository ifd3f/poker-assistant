use compact_poker::{SCard, SHand};
use itertools::Itertools;
use poker::{Evaluator};
use poker_assistant_codegen::LOOKUP;
use rand::{seq::SliceRandom, Rng};
use smallvec::{smallvec, SmallVec};

use crate::HandLookup;

use super::model::{Game, HandVec, OtherPlayer, Player};

pub struct SimParams<'a> {
    pub player: &'a OtherPlayer,
    pub sample_deck: &'a [SCard],
    pub community: &'a [SCard],
}

pub struct SimResult {
    pub sampled_hole: HandVec,
    pub best_hand: SHand,
    pub score: u32,
}

impl SimParams<'_> {
    pub fn run(&self, mut rng: impl Rng) -> SimResult {
        // Generate sampled hole
        let sampled_hole = self
            .sample_deck
            .choose_multiple(&mut rng, self.player.hole)
            .copied()
            .collect::<HandVec>();

        let mut cards = sampled_hole.clone();
        cards.extend(self.community.iter().copied());
        cards.extend(self.player.stud.iter().copied());

        let (best_hand, score) = score_hand(&cards[..]);

        SimResult {
            sampled_hole,
            best_hand,
            score,
        }
    }
}

/// Panics if provided hand is empty. Returns (card, score)
pub fn score_hand(hand: &[SCard]) -> (SHand, u32) {
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

/// This only works for 1 <= choose <= 7.
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
        _ => unimplemented!(),
    }

    dst
}

#[cfg(test)]
mod tests {
    use super::*;
    use smallvec::{smallvec, SmallVec};

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
