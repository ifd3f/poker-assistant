mod state;
mod commands;

use std::{io, str::FromStr};

use clap::{Parser, ValueEnum};
use commands::{Command, ask_command};
use compact_poker::SCard;
use poker::Card;
use poker_assistant::{
    game_repr::{self, get_deals, Deal, Round},
    prediction::model::Game,
};
use state::TuiState;

#[derive(Parser)]
pub struct Args {
    /// Type of game we are playing.
    #[clap(short = 't', long = "type")]
    pub game_type: GameType,

    /// Names of opponents you are playing against.
    ///
    /// Provide like `-o Astrid -o Alia -o Beka ...`
    #[clap(short = 'o')]
    pub opponents: Vec<String>,
}

#[derive(ValueEnum, Clone)]
pub enum GameType {
    FiveCardDraw,
    FiveCardStud,
    TexasHoldem,
}

impl GameType {
    pub fn to_rounds(&self) -> Vec<game_repr::Round> {
        match self {
            GameType::TexasHoldem => game_repr::holdem(),
            GameType::FiveCardStud => game_repr::five_card_stud(),
            GameType::FiveCardDraw => game_repr::five_card_draw(),
        }
    }
}

fn main() {
    let args = Args::parse();

    let state = TuiState::new(args.game_type.to_rounds(), args.opponents.clone());

    loop {
        let command = ask_command("poker-assistant =>");
    }
}

