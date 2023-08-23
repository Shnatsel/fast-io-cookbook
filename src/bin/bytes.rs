use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_file_path = std::env::args_os().nth(1).expect("No input file specified!");

    let test_file = std::fs::File::open(test_file_path)?;
    let reader = std::io::BufReader::new(test_file);

    let sum: u64 = reader.bytes().map(|i| i.unwrap() as u64).sum();
    println!("{sum}");
    Ok(())
}