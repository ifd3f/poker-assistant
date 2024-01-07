use std::{iter::Sum, ops::Add};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(tag = "type")]
pub enum Round {
    Deal(Deal),
    Exchange { max: u8 },
    Bet,
}

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
pub struct Deal {
    #[serde(default)]
    pub stud: u8,

    #[serde(default)]
    pub hole: u8,

    #[serde(default)]
    pub community: u8,
}

impl Add for Deal {
    type Output = Deal;

    fn add(self, rhs: Self) -> Self::Output {
        Deal {
            stud: self.stud + rhs.stud,
            hole: self.hole + rhs.hole,
            community: self.community + rhs.community,
        }
    }
}

impl Sum for Deal {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Default::default(), |a, d| a + d)
    }
}

pub fn get_deals(rounds: impl IntoIterator<Item = Round>) -> impl IntoIterator<Item = Deal> {
    rounds.into_iter().filter_map(|r| match r {
        Round::Deal(d) => Some(d),
        _ => None,
    })
}

pub fn five_card_draw() -> Vec<Round> {
    vec![
        Round::Deal(Deal {
            hole: 5,
            ..Default::default()
        }),
        Round::Bet,
        Round::Exchange { max: 3 },
        Round::Bet,
    ]
}

pub fn holdem() -> Vec<Round> {
    vec![
        Round::Deal(Deal {
            hole: 2,
            ..Default::default()
        }),
        Round::Bet,
        Round::Deal(Deal {
            community: 3,
            ..Default::default()
        }),
        Round::Bet,
        Round::Deal(Deal {
            community: 1,
            ..Default::default()
        }),
        Round::Bet,
        Round::Deal(Deal {
            community: 1,
            ..Default::default()
        }),
        Round::Bet,
    ]
}

pub fn five_card_stud() -> Vec<Round> {
    vec![
        Round::Deal(Deal {
            hole: 1,
            stud: 1,
            ..Default::default()
        }),
        Round::Bet,
        Round::Deal(Deal {
            stud: 1,
            ..Default::default()
        }),
        Round::Bet,
        Round::Deal(Deal {
            stud: 1,
            ..Default::default()
        }),
        Round::Bet,
        Round::Deal(Deal {
            stud: 1,
            ..Default::default()
        }),
        Round::Bet,
    ]
}
