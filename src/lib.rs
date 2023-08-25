use std::error::Error as StdError;
use std::panic::Location;
use std::fmt;

pub trait MyError: StdError {
    fn location(&self) -> &'static Location<'static>;
    fn inner_error(&self) -> &(dyn StdError + 'static);
}

#[derive(Debug)]
pub struct MyErrorInner<E> {
    error: E,
    location: &'static Location<'static>,
}

impl<E: StdError + Send + Sync + 'static> From<E> for Box<dyn MyError + Send + Sync> {
    #[track_caller]
    #[inline]
    fn from(error: E) -> Self {
        Box::new(MyErrorInner {
            error,
            location: Location::caller(),
        })
    }
}

impl<E: StdError + Send + 'static> From<E> for Box<dyn MyError + Send> {
    #[track_caller]
    #[inline]
    fn from(error: E) -> Self {
        Box::new(MyErrorInner {
            error,
            location: Location::caller(),
        })
    }
}

impl<E: StdError + 'static> From<E> for Box<dyn MyError> {
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

impl<E: StdError + 'static> MyError for MyErrorInner<E> {
    fn location(&self) -> &'static Location<'static> {
        self.location
    }
    fn inner_error(&self) -> &(dyn StdError + 'static) {
        &self.error
    }
}