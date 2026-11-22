// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![warn(missing_docs)]

//! # which
//!
//! A Rust implementation of `which` built for Windows first but cross-platform in theory.
//!
//! ## MSRV
//!
//! The minimum supported Rust version is:
//!
//! | Version | Edition | MSRV |
//! | --- | --- | --- |
//! | `<= 0.1.0` | 2024 | 1.85.0 |
//!
//! ## Usage Example
//!
//! ```rust,no_run
//! let found = which::which("python").unwrap();
//! // returns ["path/to/python.exe", "path/to/python3.exe"]
//! ```
//!
//! ## Benchmarks
//!
//! ```text
//! Timer precision: 100 ns
//! which                fastest       │ slowest       │ median        │ mean          │ samples │ iters
//! ├─ which                           │               │               │               │         │
//! │  ├─ t=1            1.664 ms      │ 3.346 ms      │ 1.946 ms      │ 2.009 ms      │ 100     │ 100
//! │  ├─ t=4            6.852 ms      │ 10.48 ms      │ 7.561 ms      │ 7.738 ms      │ 100     │ 100
//! │  ├─ t=8            13.69 ms      │ 15.24 ms      │ 14.41 ms      │ 14.47 ms      │ 104     │ 104
//! │  ╰─ t=16           26.44 ms      │ 30.21 ms      │ 28.01 ms      │ 28.21 ms      │ 112     │ 112
//! ╰─ which_from_paths                │               │               │               │         │
//!    ├─ t=1            177.2 µs      │ 431.7 µs      │ 229.4 µs      │ 236.8 µs      │ 100     │ 100
//!    │                 11.28 Kitem/s │ 4.631 Kitem/s │ 8.714 Kitem/s │ 8.443 Kitem/s │         │
//!    ├─ t=4            610.9 µs      │ 1.607 ms      │ 738.1 µs      │ 791.2 µs      │ 100     │ 100
//!    │                 3.273 Kitem/s │ 1.244 Kitem/s │ 2.709 Kitem/s │ 2.527 Kitem/s │         │
//!    ├─ t=8            617.2 µs      │ 1.615 ms      │ 1.227 ms      │ 1.248 ms      │ 104     │ 104
//!    │                 3.239 Kitem/s │ 1.237 Kitem/s │ 1.629 Kitem/s │ 1.601 Kitem/s │         │
//!    ╰─ t=16           1.034 ms      │ 2.886 ms      │ 2.643 ms      │ 2.578 ms      │ 112     │ 112
//!                      1.932 Kitem/s │ 692.8 item/s  │ 756.5 item/s  │ 775.5 item/s  │         │
//! ```
//!
//! ## Audits
//!
//! | Auditor | Audit Date | Version | Vulnerabilities |
//! | --- | --- | --- | --- |
//! | [cargo-audit](https://crates.io/crates/cargo-audit) | 2026-11-22 | 0.1.0 | 0 |
//!
//! ## License
//!
//! This project is licensed under the [Mozilla Public License, version 2.0](https://www.mozilla.org/en-US/MPL/2.0/).

mod error;

pub use crate::error::{Error, ErrorKind};

use crate::error::Result;
use is_executable::is_executable;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use std::{
    collections::HashSet,
    env::{split_paths, var, var_os},
    path::{Path, PathBuf},
};

/// Search for an executable in the `PATH`
///
/// ## Arguments
///
/// * `pattern` - The name of the executable to search for
///
/// ## Returns
///
/// * `Vec<PathBuf>` - The paths to the executables found
///
/// ## Errors
///
/// * [`Error::EmptyPattern`] - If the `pattern` is empty
/// * [`Error::NoPath`] - If the `PATH` environment variable is not set
/// * [`Error::IO`] - If an IO error occurs
///
/// ## Examples
///
/// ```rust,no_run
/// let found = which::which("py").unwrap();
/// // returns ["path/to/dir1/py.exe", "path/to/dir2/py.exe"]
/// ```
pub fn which<P>(pattern: P) -> Result<Vec<PathBuf>>
where
    P: AsRef<str>,
{
    fn inner(pattern: &str) -> Result<Vec<PathBuf>> {
        if pattern.trim().is_empty() {
            return Err(Error::EmptyPattern);
        }

        let paths_str = var_os("PATH").ok_or(Error::NoPath)?;
        let all_paths: Vec<PathBuf> = split_paths(&paths_str).collect();

        Ok(which_from_paths(pattern, &all_paths))
    }

    inner(pattern.as_ref())
}

/// Search for an executable in the provided `all_paths`
///
/// ## Arguments
///
/// * `pattern` - The name of the executable to search for
/// * `all_paths` - A slice of `PathBuf` representing the directories to search in
///
/// ## Returns
///
/// * `Vec<PathBuf>` - The paths to the executables found
///
/// ## Examples
///
/// ```rust,no_run
/// use std::path::PathBuf;
///
/// let paths = vec![PathBuf::from("path/to/dir1"), PathBuf::from("path/to/dir2")];
/// let found = which::which_from_paths("py", &paths);
/// // returns ["path/to/dir1/py.exe", "path/to/dir2/py.exe"]
/// ```
#[must_use]
pub fn which_from_paths<A, P>(pattern: P, all_paths: A) -> Vec<PathBuf>
where
    A: AsRef<[PathBuf]>,
    P: AsRef<str>,
{
    fn inner(pattern: &str, all_paths: &[PathBuf]) -> Vec<PathBuf> {
        let pathext = var("PATHEXT").unwrap_or_else(|_| ".COM;.EXE;.BAT;.CMD".to_string());
        let extensions: Vec<&str> = pathext.split(';').collect();
        let pattern_path = Path::new(pattern);
        let has_extension = pattern_path.extension().is_some();
        let ext_count = if has_extension {
            1
        } else {
            1 + extensions.len()
        };

        let mut all_found: Vec<((usize, usize), PathBuf)> = all_paths
            .into_par_iter()
            .enumerate()
            .flat_map(move |(path_index, dir)| {
                let mut candidates = Vec::with_capacity(ext_count);
                candidates.push(dir.join(pattern_path));

                if !has_extension {
                    for ext in &extensions {
                        let clean_ext = ext.trim_start_matches('.');
                        if !clean_ext.is_empty() {
                            candidates.push(
                                dir.join(pattern_path)
                                    .with_extension(clean_ext.to_lowercase().as_str()),
                            );
                        }
                    }
                }

                candidates
                    .into_par_iter()
                    .enumerate()
                    .map(move |(sub_index, p)| ((path_index, sub_index), p))
            })
            .filter(|(_, p)| is_executable(p))
            .collect();

        all_found.sort_by_key(|&(key, _)| key);

        let mut seen = HashSet::new();

        all_found
            .into_iter()
            .filter_map(|(_, p)| {
                if seen.contains(p.as_path()) {
                    None
                } else {
                    seen.insert(p.clone());
                    Some(p)
                }
            })
            .collect()
    }

    inner(pattern.as_ref(), all_paths.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::write;
    use tempfile::tempdir;

    #[test]
    fn test_which_deduplicates() {
        let dir1 = tempdir().unwrap();
        let dir2 = tempdir().unwrap();

        let path1 = dir1.path().join("myapp.exe");
        let path2 = dir2.path().join("myapp.exe");
        write(&path1, b"first").unwrap();
        write(&path2, b"second").unwrap();

        let paths = vec![
            dir1.path().to_path_buf(),
            dir2.path().to_path_buf(),
            dir1.path().to_path_buf(),
        ];
        let result = which_from_paths("myapp", &paths);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0], path1);
        assert_eq!(result[1], path2);
    }

    #[test]
    fn test_which_empty_pattern() {
        assert_eq!(which("").unwrap_err().kind(), ErrorKind::EmptyPattern);
        assert_eq!(which("   ").unwrap_err().kind(), ErrorKind::EmptyPattern);
    }

    #[test]
    fn test_which_finds_executable() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("testapp.exe");
        write(&file_path, b"dummy content").unwrap();

        let paths = vec![dir.path().to_path_buf()];
        let result = which_from_paths("testapp", &paths);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], file_path);
    }

    #[test]
    fn test_which_ignores_non_executable() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("testapp.txt");
        write(&file_path, b"dummy content").unwrap();

        let paths = vec![dir.path().to_path_buf()];
        let result = which_from_paths("testapp", &paths);

        assert_eq!(result, [] as [PathBuf; 0]);
    }

    #[test]
    fn test_which_respects_order() {
        let dir1 = tempdir().unwrap();
        let dir2 = tempdir().unwrap();

        let path1 = dir1.path().join("myapp.exe");
        let path2 = dir2.path().join("myapp.exe");

        write(&path1, b"first").unwrap();
        write(&path2, b"second").unwrap();

        let paths = vec![dir2.path().to_path_buf(), dir1.path().to_path_buf()];
        let result = which_from_paths("myapp", &paths);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0], path2);
        assert_eq!(result[1], path1);
    }

    #[test]
    fn test_which_with_extension() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("testapp.exe");
        write(&file_path, b"dummy content").unwrap();

        let paths = vec![dir.path().to_path_buf()];
        let result = which_from_paths("testapp.exe", &paths);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], file_path);
    }
}
