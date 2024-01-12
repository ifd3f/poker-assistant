use std::{
    cell::RefCell,
    fs::{read_to_string, File},
    io::Write,
    path::PathBuf,
};

use clap::Parser;
use compact_poker::SCard;
use dsl::{evaluate_directives, parse_program_from_str};
use plotters::{
    backend::BitMapBackend,
    chart::ChartBuilder,
    coord::{combinators::IntoLinspace, ranged1d::IntoSegmentedCoord},
    drawing::IntoDrawingArea,
    series::Histogram,
    style::{Color, RED, WHITE},
};
use poker_assistant::prediction::{model::PartialHand, montecarlo::SimParams};
use poker_assistant_lookup::N_HANDS;
use rand::{rngs::SmallRng, SeedableRng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

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

    /// Output file
    #[clap(short, long)]
    pub out: PathBuf,

    /// Number of samples to simulate
    #[clap(short = 'n', default_value = "10000")]
    pub samples: u64,
}

#[derive(clap::Args, Clone)]
pub struct TemplateArgs {
    /// File to write to. If not provided, writes to stdout.
    #[clap(short, long)]
    pub out: Option<PathBuf>,

    /// Template to use
    #[clap(short = 't')]
    pub template: BuiltinTemplates,
}

#[derive(clap::ValueEnum, Clone)]
pub enum BuiltinTemplates {
    #[clap(name = "5-card-draw")]
    FiveCardDraw,

    #[clap(name = "5-card-stud")]
    FiveCardStud,

    #[clap(name = "7-card-stud")]
    SevenCardStud,

    #[clap(name = "texas-holdem")]
    TexasHoldem,
}

impl BuiltinTemplates {
    pub fn file_contents(&self) -> &'static str {
        match self {
            BuiltinTemplates::FiveCardDraw => include_str!("templates/5-card-draw.sexp"),
            BuiltinTemplates::FiveCardStud => include_str!("templates/5-card-stud.sexp"),
            BuiltinTemplates::SevenCardStud => include_str!("templates/7-card-stud.sexp"),
            BuiltinTemplates::TexasHoldem => include_str!("templates/texas-holdem.sexp"),
        }
    }
}

fn main() {
    let args = Args::parse();

    match args.subcommand {
        Subcommand::Simulate(args) => {
            simulate(args).expect("Failed to run simulation");
        }
        Subcommand::Template(args) => {
            if let Some(p) = args.out {
                eprintln!("Writing template to {}", p.to_string_lossy());
                let mut f = File::create(p).expect("Failed to open file");
                write!(&mut f, "{}", args.template.file_contents())
                    .expect("Failed to write templates to file");
            } else {
                println!("{}", args.template.file_contents());
            }
        }
    }
}

fn simulate(args: SimulateArgs) -> anyhow::Result<()> {
    std::thread_local! {
        static RNG: RefCell<SmallRng> = RefCell::new(SmallRng::from_entropy());
    }

    let src = read_to_string(args.file)?;
    let program = parse_program_from_str(&src)?;
    let eval = evaluate_directives(&program)?;

    let mut deck = SCard::deck().collect::<Vec<_>>();
    deck.retain(|c| !eval.discarded.contains(c));

    let sims = eval
        .hands
        .into_values()
        .filter(|p| p.should_plot)
        .map(|p| {
            (
                p.name,
                SimParams {
                    player: PartialHand {
                        drawn: p.known_cards.into_iter().collect(),
                        undrawn: p.n_holes as u8,
                    },
                    sample_deck: &deck,
                },
            )
        })
        .collect::<Vec<_>>();

    let root = BitMapBackend::new(&args.out, (640, 240 * sims.len() as u32)).into_drawing_area();
    root.fill(&WHITE)?;

    let regions = root.split_evenly((sims.len(), 1));

    for (region, (name, sim_params)) in regions.iter().zip(&sims) {
        eprintln!("Simulating {}", name);

        let mut raw_results = (0..args.samples)
            .into_par_iter()
            .map(|_| RNG.with_borrow_mut(|rng| sim_params.run(rng).score))
            .collect::<Vec<_>>();
        raw_results.sort();

        let results = raw_results
            .iter()
            .map(|sr| *sr as f32 / N_HANDS as f32)
            .collect::<Vec<_>>();

        let histogram = collect_histogram(100, results.iter().copied());
        let max = *histogram.iter().max().unwrap() as f32 / args.samples as f32;
        let mean = results.iter().copied().sum::<f32>() / results.len() as f32;
        let var = results.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / results.len() as f32;
        let p50 = results[results.len() / 2];

        let mut chart = ChartBuilder::on(&region)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .margin(10)
            .caption(
                format!(
                    "{} (u={:.04}, s={:.04}, p50={:.04})",
                    name,
                    mean,
                    var.sqrt(),
                    p50
                ),
                ("sans-serif", 25.0),
            )
            .build_cartesian_2d(
                (0f32..1f32).step(0.01).use_round().into_segmented(),
                0f32..max,
            )?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .bold_line_style(WHITE.mix(0.3))
            .y_desc("Density")
            .x_desc("Quantile")
            .axis_desc_style(("sans-serif", 15))
            .draw()?;

        let histogram = Histogram::vertical(&chart)
            .style(RED.mix(0.8).filled())
            .margin(0)
            .data(results.iter().map(|s| (*s, 1.0 / args.samples as f32)));

        chart.draw_series(histogram)?;
    }

    Ok(())
}

pub fn collect_histogram(n_bins: usize, values: impl IntoIterator<Item = f32>) -> Vec<usize> {
    let mut bins = vec![0; n_bins];

    for v in values {
        let bin = (v * n_bins as f32) as usize;
        bins[bin] += 1;
    }

    bins
}
