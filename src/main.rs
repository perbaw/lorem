use clap::{Parser, ValueEnum};
use rand::seq::SliceRandom;
use rand::thread_rng;

struct Args {
    unit: Unit,
    count: usize,
}

enum Unit {
    Words,
    Sentences,
    Paragraphs,
    Bytes
}

fn main() {
    
}