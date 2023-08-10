use std::io::Write;
use linereader; // external crate `linereader`

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_file_path = std::env::args_os().nth(1).expect("No input file specified!");

    let test_file = std::fs::File::open(test_file_path)?;
    let stdout = std::io::stdout();
    // Lock the stdout once so we don't have to acquire the lock for every line, which is slow
    let mut stdout = stdout.lock();

    // Cheat by finding the *last* line break instead of the first.
    // This allows writing in big batches
    let mut reader = linereader::LineReader::with_capacity(65536, test_file);
    while let Some(batch) = reader.next_batch() {
        stdout.write_all(batch?)?;
    }

    Ok(())
}