use compact_poker::shrink_hand;
use std::{
    env,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use itertools::Itertools;
use poker::{Card, Evaluator};
use rayon::slice::ParallelSliceMut;

fn main() {
    eprintln!("generating all hands");
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

    eprintln!("writing hands in order");
    write_file(&hands, "ordered_hands.bin").unwrap();
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

fn write_file(hands: &Vec<Vec<Card>>, dest: &str) -> std::io::Result<()> {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join(dest);

    if path.exists() {
        eprintln!("skipping, file already exists");
        return Ok(());
    }

    let mut file = BufWriter::new(File::create(path)?);
    for h in hands {
        file.write(&shrink_hand(h).to_be_bytes())?;
    }
    Ok(())
}
