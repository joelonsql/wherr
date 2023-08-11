use wherr::wherr;

#[wherr]
fn add_two(s1: &str, s2: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let radix = 10;
    let i1 = i64::from_str_radix(s1, radix)?;
    let i2 = i64::from_str_radix(s2, radix)?;
    Ok(i1 + i2)
}

fn main() {
    let sum1 = add_two("10", "20").unwrap();
    println!("sum1 = {}", sum1);


    match add_two("123", "not a number") {
        Ok(sum) => {
            println!("sum2 = {}", sum);
        }
        Err(e) => {
            if let Some(wherr) = e.downcast_ref::<wherr::Wherr>() {
                println!(
                    "Error at file: '{}', line: {}. Original error: {}",
                    wherr.file, wherr.line, wherr.inner
                );
            } else {
                println!("Unexpected error: {}", e);
            }
        }
    }

    let sum3 = add_two("also not a number", "456").unwrap();
    println!("sum3 = {}", sum3);
}
