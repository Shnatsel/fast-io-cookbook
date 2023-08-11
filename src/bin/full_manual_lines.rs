use std::{io::{Read, Write}, mem};
use memchr; // quickly find position of newline character, used by both `bstr` and `std::io::BufReader`

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_file_path = std::env::args_os().nth(1).expect("No input file specified!");

    let mut test_file = std::fs::File::open(test_file_path)?;

    let stdout = std::io::stdout();
    // Lock the stdout once so we don't have to acquire the lock for every line written, which is slow
    let stdout = stdout.lock();
    // And wrap it in a BufWriter, because we're going to make lots of small writes
    // and without this it would be very slow
    let mut writer = std::io::BufWriter::new(stdout);

    // Instead of reading the whole file in memory, read it in small-ish chunks that fit into CPU cache
    // This also dramatically reduces memory usage because we never keep more than 65KB in memory
    //
    // However, this code cannot handle strings longer than 65KB in length.
    // TODO: increase the buffer size if we find an extra long string to handle this case.
    let mut chunk = vec![0; 65536];
    let mut bytes_read = test_file.read(&mut chunk)?;
    while bytes_read != 0 { // 0 bytes is how the OS indicates that we reached end of file
        let mut search_start: usize = 0;
        let search_end = bytes_read;
        loop {
            match memchr::memchr('\n' as u8, &chunk[search_start..search_end]) {
                Some(match_pos) => {
                    // get the line
                    let mut line = &chunk[search_start..search_start+match_pos];
                    // withstand wacky Windows \r\n line endings
                    if line.last() == Some(&('\r' as u8)) {
                        line = &line[..line.len() - 1];
                    }
                    // compute and output the sum
                    let sum = line.iter().map(|i| *i as u64).sum::<u64>();
                    writeln!(writer, "{sum}")?;
                    // update the search start for the next iteration
                    search_start += match_pos + 1;
                },
                None => { // No more complete lines in the buffer, handle the remainder
                    if search_start == 0 {
                        // We haven't found a single line ending. Double the buffer size and try again.
                        let mut new_buf = vec![0; chunk.len() * 2];
                        (&mut new_buf[..bytes_read]).copy_from_slice(&chunk[..bytes_read]);
                        test_file.read(&mut new_buf[bytes_read..])?;
                        _ = mem::replace(&mut chunk, new_buf);
                    } else {
                        let leftovers_len = search_end - search_start;
                        let (lines, no_lines) = chunk.split_at_mut(search_start);
                        let dest = &mut lines[..leftovers_len];
                        let src = &no_lines[..leftovers_len];
                        dest.copy_from_slice(src);
                    }
                    break
                },
            }
        }
        bytes_read = test_file.read(&mut chunk)?;
    }
    Ok(())
}