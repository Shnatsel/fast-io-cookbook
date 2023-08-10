use std::io::{Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_file_path = std::env::args_os().nth(1).expect("No input file specified!");

    let stdout = std::io::stdout();
    // Lock the stdout once so we don't have to acquire the lock for every line written, which is slow
    let mut stdout = stdout.lock();
    let mut test_file = std::fs::File::open(test_file_path)?;

    // Instead of reading the whole file in memory, read it in small-ish chunks that fit into CPU cache
    // This also dramatically reduces memory usage because we never keep more than 65KB in memory
    let mut chunk = vec![0; 65536];
    let mut bytes_read = test_file.read(&mut chunk)?;
    while bytes_read != 0 { // 0 bytes is how the OS indicates that we reached end of file
        stdout.write_all(&chunk[..bytes_read])?;
        bytes_read = test_file.read(&mut chunk)?;
    }
    Ok(())
}