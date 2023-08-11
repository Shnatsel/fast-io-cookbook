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

    let mut reader = std::io::BufReader::new(test_file);
    let mut line = String::with_capacity(8192);
    while reader.read_line(&mut line)? != 0 { // 0 bytes read is how the OS indicates that we reached end of file
        let sum: u64 = line.as_bytes().iter().map(|i| *i as u64).sum();
        writeln!(writer, "{sum}")?;
        line.clear(); // clear the buffer after each line, or we'll end up with the whole file in memory!
    }

    Ok(())
}