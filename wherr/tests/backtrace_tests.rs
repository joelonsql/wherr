#![cfg(feature = "backtrace")]

use wherr::wherr;
use wherr::GenericWherrError;

const ERROR_MESSAGE: &str = "Test error";

#[test]
fn test_backtrace() {
    // Act
    let result = f1();

    // Assert
    assert!(result.is_err());
    let wherr = result.unwrap_err();
    assert_eq!(wherr.locations().len(), 2);
    let error_message = wherr.to_string();
    assert_eq!(error_message.replace("\\", "/"), format!("{ERROR_MESSAGE}\nat wherr/tests/backtrace_tests.rs:29\nat wherr/tests/backtrace_tests.rs:23"));
}

#[wherr]
fn f1() -> Result<(), GenericWherrError> {
    f2()?;
    Ok(())
}

#[wherr]
fn f2() -> Result<(), GenericWherrError> {
    Err(std::io::Error::new(std::io::ErrorKind::Other, ERROR_MESSAGE))?;
    Ok(())
}