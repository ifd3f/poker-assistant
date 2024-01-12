use std::path::PathBuf;

use clap::{Parser, ValueEnum};

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
}

#[derive(clap::Args, Clone)]
pub struct TemplateArgs {
}


fn main() {
    let args = Args::parse();
}

