use std::io::{Write, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_file_path = std::env::args_os().nth(1).expect("No input file specified!");

    let test_file = std::fs::File::open(test_file_path)?;
    let stdout = std::io::stdout();
    // Lock the stdout once so we don't have to acquire the lock for every line written, which is slow
    let mut stdout = stdout.lock();

    let reader = std::io::BufReader::new(test_file);
    for line in reader.lines() {
        let line = &line?;
        let bytes = line.as_bytes();
        let sum: u64 = bytes.iter().map(|i| *i as u64).sum();
        writeln!(stdout, "{sum}")?;
    }

    Ok(())
}