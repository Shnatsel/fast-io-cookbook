use std::io::Write;
use bstr::io::BufReadExt; // external crate `bstr`

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
    reader.for_byte_line(|line| {
        let _validated_utf8: Result<&str, std::str::Utf8Error> = std::str::from_utf8(line);
        let sum: u64 = line.iter().map(|i| *i as u64).sum();
        writeln!(writer, "{sum}")?;
        Ok(true)
    })?;

    Ok(())
}