use divan::black_box;
use std::path::PathBuf;
use which as lib;

const BIN: &str = "python";

fn main() {
    divan::main();
}

#[divan::bench(threads = [1, 4, 8, 16])]
fn which() -> Vec<PathBuf> {
    lib::which(black_box(BIN)).unwrap()
}
