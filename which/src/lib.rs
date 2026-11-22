// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![warn(missing_docs)]

//! # which

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
/// use which::which;
///
/// let found = which("py").unwrap();
/// // returns ["py.exe", "python.exe", "python3.exe"]
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

pub(crate) fn which_from_paths(pattern: &str, all_paths: &[PathBuf]) -> Vec<PathBuf> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::write;
    use tempfile::tempdir;

    #[test]
    fn test_which_from_paths_finds_executable() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("testapp.exe");
        write(&file_path, b"dummy content").unwrap();

        let paths = vec![dir.path().to_path_buf()];
        let result = which_from_paths("testapp", &paths);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], file_path);
    }

    #[test]
    fn test_which_from_paths_respects_order() {
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
}
