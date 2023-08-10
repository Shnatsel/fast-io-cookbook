use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_file_path = std::env::args_os().nth(1).expect("No input file specified!");

    // Read the file contents
    let contents = std::fs::read(test_file_path)?;

    // Write the contents to stdout
    let mut stdout = std::io::stdout();
    stdout.write_all(&contents)?;
    Ok(())
}