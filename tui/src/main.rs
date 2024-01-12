use std::{path::PathBuf, fs::File, io::Write};

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
    #[clap(name = "sim")]
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

    /// Template to use
    #[clap(short = 't')]
    pub template: BuiltinTemplates,
}

#[derive(clap::ValueEnum, Clone)]
pub enum BuiltinTemplates {
    FiveCardDraw,
    FiveCardStud,
    TexasHoldem,
}

impl BuiltinTemplates {
    pub fn file_contents(&self) -> &'static str {
        match self {
            BuiltinTemplates::FiveCardDraw => include_str!("templates/five-card-draw.sexp"),
            BuiltinTemplates::FiveCardStud => include_str!("templates/five-card-stud.sexp"),
            BuiltinTemplates::TexasHoldem => include_str!("templates/texas-holdem.sexp"),
        }
    }
}


fn main() {
    let args = Args::parse();

    match args.subcommand {
        Subcommand::Simulate(args) => todo!(),
        Subcommand::Template(args) => {
            eprintln!("Writing template to {}", args.file.to_string_lossy());
            let mut f = File::create(args.file).expect("Failed to open file");
            write!(&mut f, "{}", args.template.file_contents()).expect("Failed to write templates to file");
        }
    }
}

