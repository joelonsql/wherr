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
pub struct Wherr {
    pub inner: Box<dyn std::error::Error>,
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
    pub fn new(err: Box<dyn std::error::Error>, file: &'static str, line: u32) -> Self {
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
            "Error at {}:{}. Original error: {}",
            self.file, self.line, self.inner
        )
    }
}

impl fmt::Debug for Wherr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error at {}:{}. Original error: {:?}",
            self.file, self.line, self.inner
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
pub fn wherrapper<T, E>(
    result: Result<T, E>,
    file: &'static str,
    line: u32,
) -> Result<T, Box<dyn std::error::Error>>
where
    E: Into<Box<dyn std::error::Error>>,
{
    match result {
        Ok(val) => Ok(val),
        Err(err) => {
            let boxed_err: Box<dyn std::error::Error> = err.into();
            if boxed_err.is::<Wherr>() {
                Err(boxed_err)
            } else {
                Err(Box::new(Wherr::new(boxed_err, file, line)))
            }
        }
    }
}
