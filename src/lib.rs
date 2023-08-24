use std::error::Error as StdError;
use std::panic::Location;
use std::fmt;

pub trait MyError: StdError {
    fn location(&self) -> &'static Location<'static>;
    fn inner_error<'a>(&'a self) -> &'a (dyn StdError + 'a);
}

#[derive(Debug)]
pub struct MyErrorInner<E> {
    error: E,
    location: &'static Location<'static>,
}

impl<'a, E: StdError + Send + Sync + 'a> From<E> for Box<dyn MyError + Send + Sync + 'a> {
    #[track_caller]
    #[inline]
    fn from(error: E) -> Self {
        Box::new(MyErrorInner {
            error,
            location: Location::caller(),
        })
    }
}

impl<'a, E: StdError + 'a> From<E> for Box<dyn MyError + 'a> {
    #[track_caller]
    #[inline]
    fn from(error: E) -> Self {
        Box::new(MyErrorInner {
            error,
            location: Location::caller(),
        })
    }
}

impl<'a, E: StdError + Send + 'a> From<E> for Box<dyn MyError + Send + 'a> {
    #[track_caller]
    #[inline]
    fn from(error: E) -> Self {
        Box::new(MyErrorInner {
            error,
            location: Location::caller(),
        })
    }
}

impl<E: fmt::Display> fmt::Display for MyErrorInner<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.error, self.location)
    }
}

impl<E: StdError> StdError for MyErrorInner<E> {}

impl<E: StdError> MyError for MyErrorInner<E> {
    fn location(&self) -> &'static Location<'static> {
        self.location
    }
    fn inner_error(&self) -> &dyn StdError {
        &self.error
    }
}
