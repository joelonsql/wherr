use wherr::MyError;

#[derive(Debug)]
enum FooError {
    A,
    B,
    C
}

impl std::fmt::Display for FooError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for FooError {}

fn my_error() -> Result<(), FooError> {
    _ = FooError::B;
    _ = FooError::C;
    Err(FooError::A)?
}

fn my_error_wrapped() -> Result<(), Box<dyn MyError>> {
    Err(FooError::A)?
}

fn main() {
    match my_error().unwrap_err() {
        FooError::A => { println!("got MyError::A"); }
        _ => { println!("got some other MyError variant"); }
    }

    match my_error_wrapped().unwrap_err().inner_error().downcast_ref::<FooError>() {
        Some(FooError::A) => { println!("got MyError::A"); }
        Some(_) => { println!("got some other MyError variant"); }
        None => { println!("got an unexpected error type"); }
    }

}