use std::{iter::Sum, ops::Add};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Round {
    Deal { name: Option<String>, counts: Deal },
    Exchange { name: Option<String>, max: u8 },
}

impl Round {
    pub fn name(&self) -> Option<&str> {
        match self {
            Round::Deal { name, .. } => name.as_deref(),
            Round::Exchange { name, .. } => name.as_deref(),
        }
    }
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
        Round::Deal { counts: d, .. } => Some(d),
        _ => None,
    })
}

pub fn five_card_draw() -> Vec<Round> {
    vec![
        Round::Deal {
            name: Some("Draw".to_owned()),
            counts: Deal {
                hole: 5,
                ..Default::default()
            },
        },
        Round::Exchange {
            name: Some("Exchange".to_owned()),
            max: 3,
        },
    ]
}

pub fn holdem() -> Vec<Round> {
    vec![
        Round::Deal {
            name: Some("Deal".to_owned()),
            counts: Deal {
                hole: 2,
                ..Default::default()
            },
        },
        Round::Deal {
            name: Some("Flop".to_owned()),
            counts: Deal {
                community: 3,
                ..Default::default()
            },
        },
        Round::Deal {
            name: Some("Turn".to_owned()),
            counts: Deal {
                community: 1,
                ..Default::default()
            },
        },
        Round::Deal {
            name: Some("River".to_owned()),
            counts: Deal {
                community: 1,
                ..Default::default()
            },
        },
    ]
}

pub fn five_card_stud() -> Vec<Round> {
    vec![
        Round::Deal {
            name: Some("Deal".to_owned()),
            counts: Deal {
                hole: 1,
                stud: 1,
                ..Default::default()
            },
        },
        Round::Deal {
            name: Some("Third Street".to_owned()),
            counts: Deal {
                stud: 1,
                ..Default::default()
            },
        },
        Round::Deal {
            name: Some("Fourth Street".to_owned()),
            counts: Deal {
                stud: 1,
                ..Default::default()
            },
        },
        Round::Deal {
            name: Some("Fifth Street".to_owned()),
            counts: Deal {
                stud: 1,
                ..Default::default()
            },
        },
    ]
}
