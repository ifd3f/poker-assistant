use std::{io, str::FromStr};

use clap::{Parser, ValueEnum};
use compact_poker::SCard;
use poker::Card;
use poker_assistant::{
    game_repr::{self, get_deals, Deal, Round},
    prediction::model::Game,
};

use crate::commands::Command;

pub struct TuiState {
    pub rounds: Vec<Round>,
    pub opponents: Vec<String>,
    pub current_round_index: usize,
    pub game: Game,
    pub commands: Vec<Command>,
}

impl TuiState {
    pub fn new(rounds: Vec<Round>, opponents: Vec<String>) -> TuiState {
        let game = Game::from_deals(opponents.len(), get_deals(rounds.iter().cloned()));
        Self {
            rounds,
            opponents,
            current_round_index: 0,
            game,
            commands: Command::commands(),
        }
    }

    pub fn handle_command(&mut self, cmd: Command) {
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
            Command::Status => todo!(),
            Command::Help => todo!(),
        }
    }

    pub fn advance_round(&mut self) -> Option<&Round> {
        if self.current_round_index - 1 >= self.rounds.len() {
            return None;
        }
        self.current_round_index += 1;
        Some(&self.rounds[self.current_round_index])
    }

    pub fn current_round(&self) -> &Round {
        &self.rounds[self.current_round_index]
    }
}

pub fn apply_deal(game: &mut Game, opponent_names: &[String], deal: Deal) -> std::io::Result<()> {
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
