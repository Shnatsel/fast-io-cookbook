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

    let mut reader = std::io::BufReader::with_capacity(65536, test_file);
    let mut line = Vec::with_capacity(8192);
    // 0 bytes read is how the OS indicates that we reached end of file
    while reader.read_until('\n' as u8 ,&mut line)? != 0 {
        // withstand wacky Windows \r\n line endings
        if line.last() == Some(&('\r' as u8)) {
            line.pop();
        }
        let sum: u64 = line.iter().map(|i| *i as u64).sum();
        writeln!(writer, "{sum}")?;
        line.clear(); // clear the buffer after each line, or we'll end up with the whole file in memory!
    }

    Ok(())
}