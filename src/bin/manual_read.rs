use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_file_path = std::env::args_os().nth(1).expect("No input file specified!");

    let mut test_file = std::fs::File::open(test_file_path)?;

    // Instead of reading the whole file in memory, read it in small-ish chunks that fit into CPU cache
    // This also dramatically reduces memory usage because we never keep more than 65KB in memory
    let mut chunk = vec![0; 65536];
    let mut sum: u64 = 0;
    let mut bytes_read = test_file.read(&mut chunk)?;
    while bytes_read != 0 { // 0 bytes is how the OS indicates that we reached end of file
        sum += chunk.iter().map(|i| *i as u64).sum::<u64>();
        bytes_read = test_file.read(&mut chunk)?;
    }
    println!("{sum}");
    Ok(())
}