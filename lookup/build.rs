use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use compact_poker::SHand;
use itertools::Itertools;
use poker::{Card, Evaluator};
use rayon::slice::ParallelSliceMut;

fn main() {
    eprintln!("generating all hands");

    write_ordered_hands().unwrap();
}

fn all_hands() -> Vec<Vec<Card>> {
    let deck = Card::generate_deck().collect_vec();
    deck.into_iter()
        .combinations(5)
        .map(|mut h| {
            h.sort();
            h
        })
        .collect_vec()
}

fn write_ordered_hands() -> std::io::Result<()> {
    let path = Path::new(&std::env::var_os("OUT_DIR").unwrap()).join("ordered_hands.bin");

    if path.exists() {
        eprintln!(
            "skipping making ordered hands, file exists: {}",
            path.to_string_lossy()
        );
        return Ok(());
    }

    let mut hands = all_hands();

    let evaluator = Evaluator::new();

    // note: evaluation is free :3
    eprintln!("sorting hands to calculate absolute rank");
    hands.par_sort_by(|a, b| {
        evaluator
            .evaluate(a)
            .unwrap()
            .cmp(&evaluator.evaluate(b).unwrap())
    });
    eprintln!("writing hands in order to {}", path.to_string_lossy());

    let mut file = BufWriter::new(File::create(path)?);
    for h in hands {
        let sh = SHand::from(&h[..]);
        file.write(sh.raw().to_be_bytes().as_slice())?;
    }
    Ok(())
}
