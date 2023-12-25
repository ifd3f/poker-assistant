use std::{collections::HashMap, ops::Index};

use compact_poker::SHand;
use poker::Card;
use poker_assistant_codegen::{N_HANDS, ORDERED_HANDS_RAW};

pub struct HandLookup {
    pub map: HashMap<SHand, u32, fasthash::t1ha::t1ha0::Hash64>,
}

impl HandLookup {
    pub fn new() -> Self {
        let mut map = HashMap::with_hasher(fasthash::t1ha::t1ha0::Hash64);
        map.reserve(N_HANDS);
        for i in 0..N_HANDS {
            let bytes = &ORDERED_HANDS_RAW[i..i + 4];
            let hand = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);

            // This is safe because it is guaranteed by ORDERED_HANDS_RAW.
            unsafe {
                map.insert(SHand::unsafe_from_raw(hand), i as u32);
            }
        }
        Self { map }
    }

    pub fn get(&self, i: impl Idx) -> Option<&u32> {
        self.map.get(&i.to_shand())
    }
}

impl<I: Idx> Index<I> for HandLookup {
    type Output = u32;

    fn index(&self, index: I) -> &Self::Output {
        HandLookup::get(self, index).expect("could not find value matching index")
    }
}

pub trait Idx {
    fn to_shand(&self) -> SHand;
}

impl Idx for &[Card] {
    fn to_shand(&self) -> SHand {
        SHand::from(*self)
    }
}

impl Idx for SHand {
    fn to_shand(&self) -> SHand {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_reverse_lookup_works() {
        HandLookup::new();
    }
}
