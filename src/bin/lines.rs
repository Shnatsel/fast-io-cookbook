use std::io::{Write, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_file_path = std::env::args_os().nth(1).expect("No input file specified!");

    let test_file = std::fs::File::open(test_file_path)?;
    let stdout = std::io::stdout();
    // Lock the stdout once so we don't have to acquire the lock for every line written, which is slow
    let stdout = stdout.lock();
    // And wrap it in a BufWriter, because we're going to make lots of small writes
    // and without this it would be very slow
    let mut writer = std::io::BufWriter::new(stdout);

    let reader = std::io::BufReader::new(test_file);
    for line in reader.lines() {
        let sum: u64 = line?.as_bytes().iter().map(|i| *i as u64).sum();
        writeln!(writer, "{sum}")?;
    }

    Ok(())
}