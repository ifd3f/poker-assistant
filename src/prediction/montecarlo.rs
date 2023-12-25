use itertools::Itertools;
use poker::Card;
use rand::{seq::SliceRandom, Rng};
use smallvec::{smallvec, SmallVec};

use crate::hand_lookup::HandLookup;

use super::model::{Game, HandVec, OtherPlayer, Player, CARD_SV_SIZE};

pub struct SimParams<'a> {
    pub player: &'a OtherPlayer,
    pub sample_deck: &'a [Card],
    pub community: &'a [Card],
}

pub struct SimResult {
    pub sampled_hole: HandVec,
    pub best_hand: HandVec,
    pub score: u32,
}

/*
impl SimParams<'_> {
    pub fn run(&self, mut rng: impl Rng, lookup: &HandLookup) -> SimResult {
        let sampled_hole = self
            .sample_deck
            .choose_multiple(&mut rng, self.player.hole)
            .copied()
            .collect::<HandVec>();

        let mut combinations = smallvec![];

        SimResult { sampled_hole, best_hand, score }
    }
}
*/

fn get_possible_hands<T: Copy + Clone>(
    choose_groups: &[(usize, &[T])],
) -> SmallVec<[SmallVec<[T; CARD_SV_SIZE]>; 10]> {
    let mut hands = smallvec![smallvec![]];
    for &(n, g) in choose_groups {
        let existing = std::mem::take(&mut hands);
        for prefix in existing {
            append_subsequences(&mut hands, prefix, g, n);
        }
    }
    hands
}

/// This only works for 0 <= choose <= 6.
fn append_subsequences<T: Copy + Clone>(
    dst: &mut SmallVec<[SmallVec<[T; CARD_SV_SIZE]>; 10]>,
    prefix: SmallVec<[T; CARD_SV_SIZE]>,
    items: &[T],
    choose: usize,
) {
    if choose == items.len() {
        let mut single = prefix;
        single.extend_from_slice(items);
        dst.push(single);
        return;
    }

    let mut hand = prefix;

    macro_rules! make_for_loop {
        ($card_range:expr, ) => {
            dst.push(hand.clone());
        };

        // using unary encoding to count nesting
        ($card_range:expr, $_:ident $($tail:ident)*) => {
            let range: &[T] = $card_range;
            for i in 0..range.len() {
                hand.push(range[i]);
                make_for_loop!(&range[i+1..], $( $tail )*);
                hand.pop();
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
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use super::get_possible_hands;
    use smallvec::{smallvec, SmallVec};

    #[test]
    fn get_possible_hands_works_equal_size() {
        let result = get_possible_hands(&[(2, &[1, 2])]);

        let expected: Vec<SmallVec<[i32; 8]>> = vec![smallvec![1, 2]];

        assert_eq!(result.to_vec(), expected);
    }

    #[test]
    fn get_possible_hands_works() {
        let result = get_possible_hands(&[(2, &[1, 2]), (3, &[10, 11, 12, 13, 14])]);

        let expected: Vec<SmallVec<[i32; 8]>> = vec![
            smallvec![1, 2, 10, 11, 12],
            smallvec![1, 2, 10, 11, 13],
            smallvec![1, 2, 10, 11, 14],
            smallvec![1, 2, 10, 12, 13],
            smallvec![1, 2, 10, 12, 14],
            smallvec![1, 2, 10, 13, 14],
            smallvec![1, 2, 11, 12, 13],
            smallvec![1, 2, 11, 12, 14],
            smallvec![1, 2, 11, 13, 14],
            smallvec![1, 2, 12, 13, 14],
        ];

        assert_eq!(result.to_vec(), expected);
    }
}
