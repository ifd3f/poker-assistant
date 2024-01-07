use std::io;

use clap::{Parser, ValueEnum};
use compact_poker::SCard;
use poker::Card;
use poker_assistant::{
    game_repr::{self, get_deals, Deal},
    prediction::model::Game,
};

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

    let rounds = args.game_type.to_rounds();
    let game = Game::from_deals(args.opponents.len(), get_deals(rounds.iter().copied()));

    for r in rounds {}
}

fn apply_deal(game: &mut Game, opponents: Vec<String>, deal: Deal) -> std::io::Result<()> {
    if deal.community > 0 {
        ask_exactly_n_cards(
            &format!("{} community cards: ", deal.hole),
            deal.community as usize,
        );
    }

    if deal.hole > 0 {
        ask_exactly_n_cards(
            &format!("{} hole cards you drew: ", deal.hole),
            deal.hole as usize,
        );
    }

    if deal.stud > 0 {
        ask_exactly_n_cards(
            &format!("{} hole cards you drew: ", deal.hole),
            deal.hole as usize,
        );
    }

    Ok(())
}

fn ask_exactly_n_cards(prompt: &str, n_cards: usize) -> std::io::Result<Vec<SCard>> {
    let mut buf: String = String::new();
    loop {
        buf.clear();
        io::stdin().read_line(&mut buf)?;
        let parsed: Result<Vec<_>, _> = buf
            .split_whitespace()
            .map(|s| s.parse::<Card>().map(|c| SCard::from(c)))
            .collect();

        let cards = match parsed {
            Ok(cards) => cards,
            Err(err) => {
                println!("failed to parse, try again: {err}");
                continue;
            }
        };

        if cards.len() != n_cards {
            println!("incorrect number of cards, expected {n_cards}");
            continue;
        }

        break Ok(cards);
    }
}
