// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Represents errors that can happen when using this crate
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Search pattern cannot be empty
    #[error("search pattern cannot be empty")]
    EmptyPattern,

    /// IO error wrapper
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    /// PATH is not set
    #[error("PATH is not set")]
    NoPath,
}

impl Error {
    /// Returns a more basic enum representing the error variants
    ///
    /// ## Returns
    ///
    /// A [`ErrorKind`]
    #[must_use]
    pub fn kind(&self) -> ErrorKind {
        match self {
            Error::EmptyPattern => ErrorKind::EmptyPattern,
            Error::IO(err) => ErrorKind::Io(err.kind()),
            Error::NoPath => ErrorKind::NoPath,
        }
    }
}

/// Represents a more basic version of the errors in [Error]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// Search pattern cannot be empty
    EmptyPattern,

    /// IO error wrapper
    Io(std::io::ErrorKind),

    /// PATH is not set
    NoPath,
}

impl<E> From<E> for ErrorKind
where
    E: AsRef<Error>,
{
    fn from(value: E) -> Self {
        let e = value.as_ref();
        e.kind()
    }
}
