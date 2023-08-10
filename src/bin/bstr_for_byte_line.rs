use std::io::Write;
use bstr::io::BufReadExt; // external crate `bstr`

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_file_path = std::env::args_os().nth(1).expect("No input file specified!");

    let test_file = std::fs::File::open(test_file_path)?;
    let stdout = std::io::stdout();
    // Lock the stdout once so we don't have to acquire the lock for every line written, which is slow
    let mut stdout = stdout.lock();

    let mut reader = std::io::BufReader::new(test_file);
    reader.for_byte_line(|line| {
        let sum: u64 = line.iter().map(|i| *i as u64).sum();
        write!(stdout, "{sum}")?;
        Ok(true)
    })?;

    Ok(())
}