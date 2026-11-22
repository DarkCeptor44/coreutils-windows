// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

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
