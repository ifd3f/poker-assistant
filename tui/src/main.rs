use std::{io, str::FromStr};

use clap::{Parser, ValueEnum};
use compact_poker::SCard;
use poker::Card;
use poker_assistant::{
    game_repr::{self, get_deals, Deal, Round},
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

    let mut rounds_iter = rounds.iter();

    let mut buf: String = String::new();
    let mut round_name: String = "Start".into();

    loop {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        match buf.parse::<Command>() {
            Ok(cmd) => handle_command(cmd),
            Err(err) => {
                println!("failed to run: {err}");
                continue;
            }
        }
    }
}

struct TuiState {
    rounds: Vec<Round>,
    current_round_index: usize,
    game: Game,
}

impl TuiState {
    fn handle_command(&mut self, cmd: Command) {
        match cmd {
            Command::NextRound => {
                let round = match self.advance_round() {
                    Some(r) => r,
                    None => {
                        println!("No rounds left");
                        return;
                    }
                };
            }
            Command::Histogram { player } => todo!(),
        }
    }

    fn advance_round(&mut self) -> Option<Round> {
        if self.current_round_index - 1 >= self.rounds.len() {
            return None;
        }
        self.current_round_index += 1;
        Some(self.rounds[self.current_round_index])
    }

    fn current_round(&self) -> &Round {
        self.rounds[self.current_round_index]
    }
}

pub enum Command {
    /// Advance to the next round and ask for drawn cards.
    NextRound,

    /// Draw the current histograms of possibilities. If player is not provided, draw every player's histogram.
    Histogram { player: Option<String> },
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_whitespace().collect::<Vec<_>>();

        if split.len() == 0 {
            return Err("No command provided".into());
        }

        match split[0] {
            "nextround" | "nr" | "next" => Ok(Self::NextRound),
            "histogram" | "hist" | "h" => Ok(Self::Histogram {
                player: split.get(1).map(|s| s.to_string()),
            }),
            other => Err(format!("Unrecognized command {other}")),
        }
    }
}

fn apply_deal(game: &mut Game, opponent_names: &[String], deal: Deal) -> std::io::Result<()> {
    if deal.community > 0 {
        let drawn = ask_exactly_n_cards(
            &format!("{} community cards: ", deal.hole),
            deal.community as usize,
        )?;
        game.community.add_cards(drawn);
    }

    if deal.hole > 0 {
        let drawn = ask_exactly_n_cards(
            &format!("{} hole cards you drew: ", deal.hole),
            deal.hole as usize,
        )?;
        game.player.hole.add_cards(drawn);
        for p in &mut game.opponents {
            p.hole.add_cards(deal.hole as u8);
        }
    }

    if deal.stud > 0 {
        let drawn = ask_exactly_n_cards(
            &format!("{} stud cards that you drew: ", deal.hole),
            deal.hole as usize,
        )?;
        game.player.stud.add_cards(drawn);

        for (p, name) in &mut game.opponents.iter_mut().zip(opponent_names) {
            p.hole.add_cards(deal.hole as u8);
            let drawn = ask_exactly_n_cards(
                &format!("{} stud cards that you drew: ", deal.hole),
                deal.hole as usize,
            )?;
            p.stud.add_cards(drawn);
        }
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
