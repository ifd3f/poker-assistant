use std::path::PathBuf;

use clap::{Parser};

pub mod dsl;

/// Poker assistant TUI.
#[derive(Parser, Clone)]
pub struct Args {
    /// Type of game we are playing.
    #[clap(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(clap::Subcommand, Clone)]
pub enum Subcommand {
    /// Perform a simulation and output a histogram.
    Simulate(SimulateArgs),

    /// Generate a template file.
    Template(TemplateArgs),
}

#[derive(clap::Args, Clone)]
pub struct SimulateArgs {
    /// File to simulate with
    pub file: PathBuf,

    /// Number of samples to simulate
    #[clap(short = 'n')]
    pub samples: u64,
}

#[derive(clap::Args, Clone)]
pub struct TemplateArgs {
    /// File to write to
    pub file: PathBuf,
}


fn main() {
    let args = Args::parse();

    match args.subcommand {
        Subcommand::Simulate(_) => todo!(),
        Subcommand::Template(_) => todo!(),
    }
}

