//! `wherr` crate provides a way to enhance Rust errors with file and line number information.
//!
//! The main struct `Wherr` represents an error containing additional metadata about where it originated from.
//!
//! The `wherr` attribute macro, defined in the `wherr_macro` crate, is re-exported here for ease of use.

use std::fmt;

// Re-export the procedural macro from the `wherr_macro` crate.
pub use wherr_macro::wherr;

/// Represents an error that includes file and line number metadata.
///
/// This error struct wraps around any error and provides a consistent interface to access the original error
/// and the file and line where it originated from.
pub struct Wherr<E> {
    pub inner: E,
    pub file: &'static str,
    pub line: u32,
}

impl<E> Wherr<E> {
    /// Create a new `Wherr` error from the given error, file, and line.
    ///
    /// # Parameters
    /// * `err`: The original error to wrap.
    /// * `file`: The file where the error occurred.
    /// * `line`: The line number where the error occurred.
    pub fn new(err: E, file: &'static str, line: u32) -> Self {
        Wherr {
            inner: err,
            file,
            line,
        }
    }
}

impl<E: fmt::Display> fmt::Display for Wherr<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\nerror at {}:{}",
            self.inner, self.file, self.line
        )
    }
}

impl<E: fmt::Debug> fmt::Debug for Wherr<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}\nerror at {}:{}",
            self.inner, self.file, self.line
        )
    }
}

impl<E: std::error::Error> std::error::Error for Wherr<E> {}
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
pub fn wherrapper<T, E>(
    result: Result<T, E>,
    file: &'static str,
    line: u32,
) -> Result<T, Wherr<E>>
{
    match result {
        Ok(val) => Ok(val),
        Err(err) => Err(Wherr::new(err, file, line))
    }
}
