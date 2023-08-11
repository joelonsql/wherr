use wherr::wherr;

// Function to add two numbers represented as strings.
// Returns a Result with the sum within an `Ok` variant if successful,
// or an `Err` variant if there's an error.
#[wherr]
fn add(s1: &str, s2: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let radix = 10;
    let i1 = i64::from_str_radix(s1, radix)?;
    let i2 = i64::from_str_radix(s2, radix)?;
    Ok(i1 + i2)
}

fn main() {
    let x = add("123", "not a number");
    println!("x = {:?}", x);
}
