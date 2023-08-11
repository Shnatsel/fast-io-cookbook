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

    // Instead of reading the whole file in memory, read it in small-ish chunks that fit into CPU cache.
    let mut chunk = vec![0; 65536];
    let mut bytes_read = test_file.read(&mut chunk)?;
    let mut search_start: usize = 0;
    while bytes_read != 0 { // 0 bytes is how the OS indicates that we reached end of file
        let mut first_iteration = true;
        let search_end = bytes_read;
        loop {
            match memchr::memchr('\n' as u8, &chunk[search_start..search_end]) {
                Some(match_pos) => {
                    // obtain the line
                    let mut line = if ! first_iteration {
                        &chunk[search_start..search_start+match_pos]
                    } else {
                        // special handling for the first iteration of the inner loop:
                        // we do not re-scan the bytes already scanned during previous iterations
                        // of the outer loop. This avoids quadratic runtime on very long lines.
                        &chunk[..match_pos]
                    };
                    // withstand wacky Windows \r\n line endings
                    if line.last() == Some(&('\r' as u8)) {
                        line = &line[..line.len() - 1];
                    }
                    // compute and output the sum
                    let sum = line.iter().map(|i| *i as u64).sum::<u64>();
                    writeln!(writer, "{sum}")?;
                    // update the search start for the next iteration
                    first_iteration = false;
                    search_start += match_pos + 1;
                },
                None => { // No more complete lines in the buffer, handle the remainder
                    if first_iteration {
                        // We haven't found a single line ending. It must be a *really* long string.
                        // Double the buffer size and try again.

                        // vec![0; N] desugars into calloc() that gets pre-zeroed memory from the OS.
                        // This should be faster than a memset().
                        let mut new_buf = vec![0; chunk.len() * 2];
                        // Move the data we already read from the old buffer to the new, bigger one
                        (&mut new_buf[..bytes_read]).copy_from_slice(&chunk[..bytes_read]);
                        // Read more data into the new buffer to get it ready for searching
                        let more_bytes_read = test_file.read(&mut new_buf[bytes_read..])?;
                        // Replace the old buffer with the new one
                        _ = mem::replace(&mut chunk, new_buf);
                        search_start = bytes_read;
                        bytes_read = more_bytes_read;
                    } else {
                        // Move the leftovers from the end to the beginning, and fill the remaining space
                        let leftovers_len = search_end - search_start;
                        let (lines, no_lines) = chunk.split_at_mut(search_start);
                        let dest = &mut lines[..leftovers_len];
                        let src = &no_lines[..leftovers_len];
                        dest.copy_from_slice(src);
                        search_start = leftovers_len;
                        bytes_read = test_file.read(&mut chunk[leftovers_len..])?;
                    }
                    break
                },
            }
        }
    }
    Ok(())
}