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

pub fn standard() -> Vec<Round> {
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
