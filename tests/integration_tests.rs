use wherr::MyError;

fn f3() -> Result<(), Box<dyn MyError + Send + Sync>> {
    i64::from_str_radix("not a decimal number", 10)?;

    Ok(())
}

fn f2() -> Result<(), Box<dyn MyError + Send + Sync>> {
    f3()?;

    Ok(())
}

fn f1() -> Result<(), Box<dyn MyError + Send + Sync>> {
    f2()?;

    Ok(())
}

#[test]
fn test_wherr_macro() {
    match f1() {
        Ok(_) => panic!("Expected an error"),
        Err(err) => {
            assert_eq!(err.inner_error().to_string(), "invalid digit found in string");
        }
    }
}
