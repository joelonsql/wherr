extern crate wherr;

use wherr::{wherr, wherrapper, Wherr};

#[test]
fn test_wherr_new() {
    let error_message = "Test error";
    let err = std::io::Error::new(std::io::ErrorKind::Other, error_message);
    let wherr = Wherr::new(Box::new(err), "test.rs", 42);

    assert_eq!(wherr.file, "test.rs");
    assert_eq!(wherr.line, 42);
    assert_eq!(wherr.inner.to_string(), error_message);
}

#[test]
fn test_wherr_display() {
    let error_message = "Test error";
    let err = std::io::Error::new(std::io::ErrorKind::Other, error_message);
    let wherr = Wherr::new(Box::new(err), "test.rs", 42);

    assert_eq!(
        format!("{}", wherr),
        format!("{}\nerror at test.rs:42", error_message)
    );
}

#[test]
fn test_wherrapper() {
    let error_message = "Test error";
    let err = std::io::Error::new(std::io::ErrorKind::Other, error_message);
    let result: Result<(), _> = Err(err);

    match wherrapper(result, "test.rs", 42) {
        Ok(_) => panic!("Expected an error"),
        Err(wherr) => {
            assert_eq!(wherr.file, "test.rs");
            assert_eq!(wherr.line, 42);
            assert_eq!(wherr.inner.to_string(), error_message);
        }
    }
}

#[wherr]
fn f3() -> Result<(), Box<dyn std::error::Error>> {
    i64::from_str_radix("not a decimal number", 10)?;

    Ok(())
}

#[wherr]
fn f2() -> Result<(), Box<dyn std::error::Error>> {
    f3()?;

    Ok(())
}

#[wherr]
fn f1() -> Result<(), Box<dyn std::error::Error>> {
    f2()?;

    Ok(())
}

#[test]
fn test_wherr_macro() {
    match f1() {
        Ok(_) => panic!("Expected an error"),
        Err(err) => {
            let wherr = err.downcast::<Wherr>().expect("Expected a Wherr error");
            assert_eq!(wherr.file, "wherr/tests/integration_tests.rs");
            assert_eq!(wherr.line, 47);
            assert_eq!(wherr.inner.to_string(), "invalid digit found in string");
        }
    }
}
