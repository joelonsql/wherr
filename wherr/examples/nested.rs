use wherr::wherr;

#[wherr]
fn add_two(s1: &str, s2: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let radix = 10;
    let i1 = i64::from_str_radix(s1, radix)?;
    let i2 = i64::from_str_radix(s2, radix)?;
    Ok(i1 + i2)
}

fn add_four(s1: &str, s2: &str, s3: &str, s4: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let i1 = add_two(s1, s2)?;
    let i2 = add_two(s3, s4)?;
    Ok(i1 + i2)
}

fn main() {
    let sum = add_four("10", "20", "30", "foo").unwrap();
    println!("sum = {}", sum);
}
