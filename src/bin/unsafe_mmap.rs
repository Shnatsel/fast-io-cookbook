use memmap2; // external crate that provides mmap()

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_file_path = std::env::args_os().nth(1).expect("No input file specified!");
    let test_file = std::fs::File::open(test_file_path)?;

    // Zero-copy "read" of the file contents (YOLO)
    let contents = unsafe { memmap2::Mmap::map(&test_file)? };

    let sum: u64 = contents.iter().map(|i| *i as u64).sum();
    println!("{sum}");
    Ok(())
}