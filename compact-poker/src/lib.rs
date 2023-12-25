//! `poker` crate represents cards like this:
//!
//! ```text
//! Card:
//!                           bitrank     suit rank   prime
//!                     +--------+--------+--------+--------+
//!                     |xxxbbbbb|bbbbbbbb|cdhsrrrr|xxpppppp|
//!                     +--------+--------+--------+--------+
//! ```
//!
//! Cards can then be reduced into 6 bits like so:
//!
//! ```text
//!                     +--------+
//!                     |  rrrrss|
//!                     +--------+
//! ```
//!
//! so a 5-card hand can fit in 30 bits:
//!
//! ```text
//!                     +--------+--------+--------+--------+
//!                     |  card05|card04ca|rd03card|02card01|
//!                     +--------+--------+--------+--------+
//! ```
//!
//! A hand can be uniquely represented if the cards are sorted before being
//! transformed into this format. The sort key is always the shrunken representation
//! of the cards.

use itertools::Itertools;
use poker::{Card, Rank, Suit};
use variter::VarIter;

/// A reduced representation of a card.
#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SCard(u8);

impl SCard {
    #[inline]
    pub fn new(r: Rank, s: Suit) -> Self {
        Self((r as u8) << 2 | s as u8)
    }

    #[inline]
    unsafe fn unsafe_from_raw(raw: u8) -> Self {
        SCard(raw)
    }

    #[inline]
    pub fn raw(&self) -> u8 {
        self.0
    }

    pub fn deck() -> impl Iterator<Item = SCard> {
        Suit::ALL_VARIANTS
            .iter()
            .cartesian_product(Rank::ALL_VARIANTS.iter())
            .map(|(s, r)| SCard::new(*r, *s))
    }

    #[inline]
    pub fn suit(&self) -> Suit {
        let suit = self.0 & 0x3;
        unsafe { std::mem::transmute(suit) }
    }

    #[inline]
    pub fn rank(&self) -> Rank {
        let rank = self.0 >> 2;
        unsafe { std::mem::transmute(rank) }
    }
}

impl From<Card> for SCard {
    fn from(c: Card) -> Self {
        SCard::new(c.rank(), c.suit())
    }
}

impl From<SCard> for Card {
    fn from(c: SCard) -> Self {
        Card::new(c.rank(), c.suit())
    }
}

impl std::fmt::Debug for SCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SCard")
            .field("rank", &self.rank())
            .field("suit", &self.suit())
            .field("raw", &self.0)
            .finish()
    }
}

/// A reduced representation of a hand.
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct SHand(u32);

impl SHand {
    #[inline]
    pub unsafe fn unsafe_from_raw(hand: u32) -> Self {
        SHand(hand)
    }

    #[inline]
    pub fn raw(&self) -> u32 {
        self.0
    }

    #[allow(overflowing_literals)]
    #[inline]
    pub fn members(&self) -> [SCard; 5] {
        let hand = self.0;
        unsafe {
            [
                SCard::unsafe_from_raw((hand & 0x3f) as u8),
                SCard::unsafe_from_raw(((hand >> 6) & 0x3f) as u8),
                SCard::unsafe_from_raw(((hand >> 12) & 0x3f) as u8),
                SCard::unsafe_from_raw(((hand >> 18) & 0x3f) as u8),
                SCard::unsafe_from_raw(((hand >> 24) & 0x3f) as u8),
            ]
        }
    }
}

impl From<&[SCard]> for SHand {
    #[inline]
    fn from(hand: &[SCard]) -> Self {
        let mut vals = [hand[0].0, hand[1].0, hand[2].0, hand[3].0, hand[4].0];
        vals.sort();

        SHand(
            vals[0] as u32
                | ((vals[1] as u32) << 6)
                | ((vals[2] as u32) << 12)
                | ((vals[3] as u32) << 18)
                | (vals[4] as u32) << 24,
        )
    }
}

impl From<&[Card]> for SHand {
    fn from(hand: &[Card]) -> Self {
        let shrunk: &[SCard] = &[
            hand[0].into(),
            hand[1].into(),
            hand[2].into(),
            hand[3].into(),
            hand[4].into(),
        ];

        SHand::from(shrunk)
    }
}

impl FromIterator<SCard> for SHand {
    #[inline]
    fn from_iter<T: IntoIterator<Item = SCard>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let shrunk: &[SCard] = &[
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        ];
        SHand::from(shrunk)
    }
}

impl std::fmt::Debug for SHand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SHand")
            .field("raw", &self.raw())
            .field("members", &self.members())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn roundtrip_large_cards_to_scards() {
        for lc in Card::generate_deck() {
            assert_eq!(Card::from(SCard::from(lc)), lc)
        }
    }

    #[test]
    fn scards_have_right_data() {
        for lc in Card::generate_deck() {
            let sc = SCard::from(lc);
            assert_eq!((sc.rank(), sc.suit()), (lc.rank(), lc.suit()));
        }
    }

    #[test]
    fn scards_generate_correctly() {
        let lcs = Card::generate_deck()
            .map(SCard::from)
            .collect::<HashSet<_>>();
        let scs = SCard::deck().collect::<HashSet<_>>();
        assert_eq!(lcs, scs);
    }

    #[test]
    fn hand_roundtrip() {
        let orig: &[SCard] = &[
            SCard::new(Rank::Two, Suit::Hearts),
            SCard::new(Rank::Two, Suit::Diamonds),
            SCard::new(Rank::Eight, Suit::Clubs),
            SCard::new(Rank::Queen, Suit::Hearts),
            SCard::new(Rank::King, Suit::Spades),
        ];

        let h = SHand::from(orig);

        assert_eq!(h.members(), orig);
    }

    #[test]
    fn hand_ignores_order() {
        let a: &[SCard] = &[
            SCard::new(Rank::Eight, Suit::Clubs),
            SCard::new(Rank::Two, Suit::Hearts),
            SCard::new(Rank::Queen, Suit::Hearts),
            SCard::new(Rank::King, Suit::Spades),
            SCard::new(Rank::Two, Suit::Diamonds),
        ];
        let b: &[SCard] = &[
            SCard::new(Rank::Eight, Suit::Clubs),
            SCard::new(Rank::Queen, Suit::Hearts),
            SCard::new(Rank::Two, Suit::Diamonds),
            SCard::new(Rank::King, Suit::Spades),
            SCard::new(Rank::Two, Suit::Hearts),
        ];

        let ha = SHand::from(a);
        let hb = SHand::from(b);

        assert_eq!(ha, hb);
        assert_eq!(ha.raw(), hb.raw());
    }
}
