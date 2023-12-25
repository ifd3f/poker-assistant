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
//!                     |  ssrrrr|
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
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SCard(u8);

impl SCard {
    #[inline]
    pub fn new(r: Rank, s: Suit) -> Self {
        Self((s as u8) << 4 | r as u8)
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
        let suit = (self.0 & 0x30) >> 4;
        unsafe { std::mem::transmute(suit) }
    }

    #[inline]
    pub fn rank(&self) -> Rank {
        let rank = self.0 & 0xf;
        unsafe { std::mem::transmute(rank) }
    }
}

impl From<Card> for SCard {
    fn from(card: Card) -> Self {
        let suit = card.suit() as u8;
        let rank = (card.unique_integer() >> 8) as u8 & 0xf;
        SCard(rank | (suit << 4))
    }
}

impl From<SCard> for Card {
    fn from(c: SCard) -> Self {
        Card::new(c.rank(), c.suit())
    }
}

/// A reduced representation of a hand.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SHand(u32);

impl SHand {
    pub unsafe fn unsafe_from_raw(hand: u32) -> Self {
        SHand(hand)
    }

    #[allow(overflowing_literals)]
    pub fn members(hand: u32) -> [SCard; 5] {
        unsafe {
            [
                SCard::unsafe_from_raw(hand as u8),
                SCard::unsafe_from_raw((hand >> 6) as u8),
                SCard::unsafe_from_raw((hand >> 12) as u8),
                SCard::unsafe_from_raw((hand >> 18) as u8),
                SCard::unsafe_from_raw((hand >> 24) as u8),
            ]
        }
    }
}

impl From<&[SCard]> for SHand {
    fn from(hand: &[SCard]) -> Self {
        let mut shrunk = [hand[0].0, hand[1].0, hand[2].0, hand[3].0, hand[4].0];
        shrunk.sort();

        SHand(
            ((shrunk[4] as u32) << 24)
                | ((shrunk[3] as u32) << 18)
                | ((shrunk[2] as u32) << 12)
                | ((shrunk[1] as u32) << 6)
                | shrunk[0] as u32,
        )
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
        let lcs = Card::generate_deck().map(SCard::from).collect::<HashSet<_>>();
        let scs = SCard::deck().collect::<HashSet<_>>();
        assert_eq!(lcs, scs);
    }
}
