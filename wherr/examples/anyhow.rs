use anyhow::{Context, Result};
use wherr::wherr;

#[wherr]
fn concat_files(path1: &str, path2: &str) -> Result<String> {
    let mut content1 = std::fs::read_to_string(path1).with_context(|| format!("Failed to read {}", path1))?;
    let content2 = std::fs::read_to_string(path2).with_context(|| format!("Failed to read {}", path2))?;

    content1.push_str(&content2);
    Ok(content1)
}

fn main() {
    let content = concat_files("file1.txt", "file2.txt").expect("Failed to concatenate the files");
    println!("Concatenated content:\n{content}");
}
