use std::io::Write;
use bstr::io::BufReadExt; // external crate `bstr`
use simdutf8; // external crate for faster UTF-8 validation

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_file_path = std::env::args_os().nth(1).expect("No input file specified!");

    let test_file = std::fs::File::open(test_file_path)?;
    let stdout = std::io::stdout();
    // Lock the stdout once so we don't have to acquire the lock for every line, which is slow
    let mut stdout = stdout.lock();

    let mut reader = std::io::BufReader::new(test_file);
    reader.for_byte_line(|line| {
        let _validated_utf8 = simdutf8::basic::from_utf8(line);
        stdout.write_all(line)?;
        Ok(true)
    })?;

    Ok(())
}