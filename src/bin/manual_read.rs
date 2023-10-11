use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_file_path = std::env::args_os().nth(1).expect("No input file specified!");

    let mut test_file = std::fs::File::open(test_file_path)?;

    // Instead of reading the whole file in memory, read it in small-ish chunks that fit into CPU cache.
    // This also dramatically reduces memory usage because we never keep more than 64 KiB in memory.
    let mut chunk = vec![0; 65536];
    let mut sum: u64 = 0;
    loop {
        let bytes_read = test_file.read(&mut chunk)?;
        if bytes_read == 0 {
            break;
        }
        sum += chunk[..bytes_read].iter().map(|i| *i as u64).sum::<u64>();
    }
    
    println!("{sum}"); 
    Ok(()) 
}
