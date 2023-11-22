//! `wherr` crate provides a way to enhance Rust errors with file and line number information.
//!
//! The main struct `Wherr` represents an error containing additional metadata about where it originated from.
//!
//! The `wherr` attribute macro, defined in the `wherr_macro` crate, is re-exported here for ease of use.

#[cfg(feature = "backtrace")]
mod backtrace;

#[cfg(feature = "backtrace")]
pub use self::backtrace::*;

use std::fmt;

// Re-export the procedural macro from the `wherr_macro` crate.
pub use wherr_macro::wherr;

#[cfg(feature = "anyhow")]
use anyhow::Error as AnyhowError;

#[cfg(feature = "anyhow")]
type Wherror = AnyhowError;
#[cfg(not(feature = "anyhow"))]
type Wherror = Box<dyn std::error::Error>;

/// Represents an error that includes file and line number metadata.
///
/// This error struct wraps around any error and provides a consistent interface to access the original error
/// and the file and line where it originated from.
pub struct Wherr {
    pub inner: Wherror,
    pub file: &'static str,
    pub line: u32,
}

impl Wherr {
    /// Create a new `Wherr` error from the given error, file, and line.
    ///
    /// # Parameters
    /// * `err`: The original error to wrap.
    /// * `file`: The file where the error occurred.
    /// * `line`: The line number where the error occurred.
    pub fn new(err: Wherror, file: &'static str, line: u32) -> Self {
        Wherr {
            inner: err,
            file,
            line,
        }
    }
}

impl fmt::Display for Wherr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\nerror at {}:{}",
            self.inner, self.file, self.line
        )
    }
}

impl fmt::Debug for Wherr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}\nerror at {}:{}",
            self.inner, self.file, self.line
        )
    }
}

impl std::error::Error for Wherr {}

/// Utility function to wrap the given error into a `Wherr` struct, adding file and line number details.
///
/// # Parameters
/// * `result`: The result containing the error to wrap.
/// * `file`: The file where the error occurred.
/// * `line`: The line number where the error occurred.
///
/// # Returns
/// If the original error is already of type `Wherr`, it is returned as is.
/// Otherwise, the original error is wrapped inside a `Wherr` and returned.
#[cfg(not(feature = "backtrace"))]
pub fn wherrapper<T, E>(
    result: Result<T, E>,
    file: &'static str,
    line: u32,
) -> Result<T, Wherror>
where
    E: Into<Wherror>,
{
    match result {
        Ok(val) => Ok(val),
        Err(err) => {
            let error: Wherror = err.into();
            if error.is::<Wherr>() {
                Err(error)
            } else {
                Err(Wherr::new(error, file, line).into())
            }
        }
    }
}
