fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_file_path = std::env::args_os().nth(1).expect("No input file specified!");

    // Read the file contents
    let contents = std::fs::read(test_file_path)?;

    let sum: u64 = contents.iter().map(|i| *i as u64).sum();
    println!("{sum}");
    Ok(())
}