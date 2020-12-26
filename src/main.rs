use catcat::write_output;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, prelude::*};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            // No filename was provided; use stdin.
            write_output(io::stdin().lock().lines())?;
        }
        2 => {
            // Filename was provided; use that.
            write_output(io::BufReader::new(File::open(&args[1])?).lines())?;
        }
        _ => {
            eprintln!("usage: catcat [file]");
        }
    }

    Ok(())
}
