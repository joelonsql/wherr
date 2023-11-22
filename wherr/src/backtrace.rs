use std::{fmt, error::Error};

pub use wherr_macro::wherr;
pub type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type GenericWherrError = Box<dyn WherrError + Send + Sync + 'static>;

pub trait WherrError: Error {
    fn into_inner(&self) -> Option<&GenericError>;
    fn take_inner(&mut self) -> Option<GenericError>;
    fn locations(&self) -> &[Location];
    fn push_location(&mut self, file: &'static str, line: u32);
    fn stack(&self) -> String {
        let mut result = String::with_capacity(128);
        for loc in self.locations() {
            result.push_str("at ");
            result.push_str(loc.file);
            result.push(':');
            result.push_str(loc.line.to_string().as_str());
            result.push('\n');
        }
        result
    }
}

impl WherrError for WherrWithBacktrace {
    fn into_inner(&self) -> Option<&GenericError> {
        self.inner.as_ref()
    }

    fn take_inner(&mut self) -> Option<GenericError> {
        self.inner.take()
    }
    
    fn locations(&self) -> &[Location] {
        self.locations.as_slice()
    }

    fn push_location(&mut self, file: &'static str, line: u32) {
        self.locations.push(Location { file, line });
    }
}

pub struct WherrWithBacktrace {
    pub inner: Option<GenericError>,

    pub locations: Vec<Location>,
}

#[derive(Debug)]
pub struct Location {
    pub file: &'static str,
    pub line: u32,
}

impl WherrWithBacktrace {
    pub fn new(err: GenericError) -> Self {
        WherrWithBacktrace {
            inner: Some(err),
            locations: Vec::default(),
        }
    }
}

impl Default for WherrWithBacktrace {
    fn default() -> Self {
        WherrWithBacktrace {
            inner: None,
            locations: Vec::default(),
        }
    }
}

impl fmt::Display for WherrWithBacktrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for source in &self.inner {
            write!(f, "{}", source)?;
        }

        for loc in self.locations.iter() {
            write!(f, "\nat {}:{}", loc.file, loc.line)?;
        }

        Ok(())
    }
}

impl fmt::Debug for WherrWithBacktrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.inner)?;
        for loc in self.locations.iter() {
            write!(f, "\nat {}:{}", loc.file, loc.line)?;
        }

        Ok(())
    }
}

impl std::error::Error for WherrWithBacktrace { }

pub fn wherrapper<T, E>(
    result: Result<T, E>,
    file: &'static str,
    line: u32,
) -> Result<T, GenericWherrError>
where
    E: Into<GenericWherrError> + 'static,
{
    match result {
        Ok(val) => Ok(val),
        Err(err) => {
            let mut wherr_error: GenericWherrError = err.into();
            wherr_error.push_location(file, line);
            Err(wherr_error)
        }
    }
}

impl<E: Error + Send + Sync + 'static> From<E> for GenericWherrError {
    fn from(error: E) -> Self {
        Box::new(WherrWithBacktrace::new(Box::new(error)))
    }
}
