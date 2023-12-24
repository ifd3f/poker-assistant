use std::{
    cmp::Ordering,
    ops::{Index, IndexMut},
};

use itertools::Itertools;
use smallvec::SmallVec;
use strum::IntoEnumIterator;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, strum::EnumIter)]
#[repr(u8)]
pub enum Suit {
    Club = 1,
    Diamond,
    Heart,
    Spade,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, strum::EnumIter)]
#[repr(u8)]
pub enum Rank {
    R2 = 2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    RJ,
    RQ,
    RK,
    RA,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    cards: [Card; 5],
}

impl Hand {
    pub fn new(mut cards: [Card; 5]) -> Hand {
        cards.sort_by(|a, b| b.cmp(a));
        Self { cards }
    }

    pub fn iter(&self) -> impl Iterator<Item = Card> + '_ {
        self.cards.iter().copied()
    }

    pub fn is_straight(&self) -> Option<Rank> {
        if self
            .iter()
            .map(|c| c.rank as u8 - self.cards[0].rank as u8)
            .eq(0..5)
        {
            Some(self.cards[0].rank)
        } else {
            None
        }
    }

    pub fn is_flush(&self) -> Option<Suit> {
        let suit = self.cards[0].suit;
        if self.iter().all(|c| c.suit == suit) {
            Some(suit)
        } else {
            None
        }
    }

    pub fn rank_counts(&self) -> RankCounts {
        RankCounts::from_iter(self.cards)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct RankCounts {
    counts: [u8; 13],
}

impl RankCounts {
    pub fn new() -> Self {
        RankCounts { counts: [0; 13] }
    }
}

impl FromIterator<Card> for RankCounts {
    fn from_iter<I: IntoIterator<Item = Card>>(iter: I) -> Self {
        iter.into_iter().map(|c| c.rank).collect()
    }
}

impl FromIterator<Rank> for RankCounts {
    fn from_iter<I: IntoIterator<Item = Rank>>(iter: I) -> Self {
        let mut cs = RankCounts::new();
        for r in iter {
            cs[r] += 1;
        }
        cs
    }
}

impl Index<Rank> for RankCounts {
    type Output = u8;

    fn index(&self, index: Rank) -> &Self::Output {
        &self.counts[index as usize - Rank::R2 as usize]
    }
}

impl IndexMut<Rank> for RankCounts {
    fn index_mut(&mut self, index: Rank) -> &mut Self::Output {
        &mut self.counts[index as usize - Rank::R2 as usize]
    }
}

/// This structure contains all data required to rank two hands.
#[derive(Debug, PartialEq, Eq)]
pub enum HandType {
    StraightFlush(Card),
    Flush(Suit),
    Straight(Rank),
    Kind(u8, Rank),
    High(Rank),
    Tiebreaker(Card),
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use HandType::*;

        let pair = (*self, *other);

        macro_rules! dominates {
            ($p:pat) => {
                if let &($p, _) = &pair {
                    return Some(Ordering::Greater);
                } else if let &(_, $p) = &pair {
                    return Some(Ordering::Less);
                }
            };
        }

        if let (StraightFlush(l), StraightFlush(r)) = &pair {
            return Some(l.cmp(r));
        }
        dominates!(StraightFlush(_));

        if let &(Straight(l), Straight(r)) = &pair {
            return match l.cmp(r) {
                Equal => None,
                other => Some(other),
            };
        }
        dominates!(Straight(_));

        if let &(Flush(l), Flush(r)) = &pair {
            return match l.cmp(r) {
                Equal => None,
                other => Some(other),
            };
        }
        dominates!(Flush(_));

        unimplemented!()
    }
}

pub fn all_cards() -> impl Iterator<Item = Card> {
    Rank::iter()
        .cartesian_product(Suit::iter())
        .map(|(rank, suit)| Card { rank, suit })
}

pub fn all_hands() -> impl Iterator<Item = Hand> {
    all_cards().combinations(5).map(|cards| Hand {
        cards: cards.try_into().unwrap(),
    })
}

pub fn sorted_hands() -> Vec<Card> {
    todo!()
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cards.cmp(&other.cards))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let s_flush = self.is_flush();
        let s_straight = self.is_straight();
        let o_flush = self.is_flush();
        let o_straight = self.is_straight();
    }
}
