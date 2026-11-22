// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use divan::{Bencher, black_box, counter::ItemsCount};
use std::path::PathBuf;
use tempfile::tempdir;
use which as lib;

const BIN: &str = "python";

fn main() {
    divan::main();
}

#[divan::bench(threads = [1, 4, 8, 16])]
fn which() -> Vec<PathBuf> {
    lib::which(black_box(BIN)).unwrap()
}

#[divan::bench(threads = [1, 4, 8, 16])]
fn which_from_paths(b: Bencher) {
    let dir1 = tempdir().unwrap();
    let dir2 = tempdir().unwrap();

    let path1 = dir1.path().join("myapp.exe");
    let path2 = dir2.path().join("myapp.exe");

    let paths = vec![path1, path2];

    b.counter(ItemsCount::new(paths.len()))
        .bench(|| black_box(lib::which_from_paths(black_box(BIN), black_box(&paths))));
}
