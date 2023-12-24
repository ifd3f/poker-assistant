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

use poker::Card;

pub fn shrink_card(card: Card) -> u8 {
    let suit = card.suit() as u8;
    let rank = (card.unique_integer() >> 8) as u8 & 0xf;
    rank | (suit << 4)
}

pub unsafe fn unsafe_grow_card(card: u8) -> Card {
    let suit = (card & 0x30) >> 4;
    let rank = card & 0xf;
    Card::new(std::mem::transmute(rank), std::mem::transmute(suit))
}

pub fn shrink_hand(hand: &[Card]) -> u32 {
    let mut shrunk = [
        shrink_card(hand[0]),
        shrink_card(hand[1]),
        shrink_card(hand[2]),
        shrink_card(hand[3]),
        shrink_card(hand[4]),
    ];
    shrunk.sort();

    ((shrunk[4] as u32) << 24)
        | ((shrunk[3] as u32) << 18)
        | ((shrunk[2] as u32) << 12)
        | ((shrunk[1] as u32) << 6)
        | shrunk[0] as u32
}

#[allow(overflowing_literals)]
pub unsafe fn unsafe_grow_hand(hand: u32) -> [Card; 5] {
    [
        unsafe_grow_card(hand as u8),
        unsafe_grow_card((hand >> 6) as u8),
        unsafe_grow_card((hand >> 12) as u8),
        unsafe_grow_card((hand >> 18) as u8),
        unsafe_grow_card((hand >> 24) as u8),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_cards() {
        for c in Card::generate_deck() {
            unsafe { assert_eq!(unsafe_grow_card(shrink_card(c)), c) }
        }
    }
}
