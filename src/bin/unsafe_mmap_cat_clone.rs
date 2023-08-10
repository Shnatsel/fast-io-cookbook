use std::io::Write;
use memmap2; // external crate that provides mmap()

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_file_path = std::env::args_os().nth(1).expect("No input file specified!");
    let test_file = std::fs::File::open(test_file_path)?;

    // Zero-copy "read" of the file contents (YOLO)
    let contents = unsafe { memmap2::Mmap::map(&test_file)? };

    // Write the contents to stdout
    let mut stdout = std::io::stdout();
    stdout.write_all(&contents)?;
    Ok(())
}