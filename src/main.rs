use std::{fs, io::Write};
use clap::Parser;
use anyhow::Result;
use rand::{distributions::Alphanumeric, Rng};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of file to generate or append.
    #[arg(short, long)]
    path: String,

    /// Number of lines to generate.
    #[arg(short, long, default_value_t = 1000)]
    lines: usize,

    /// Number of characters in each line.
    #[arg(short, long, default_value_t = 8)]
    characters: usize,
}

fn main() -> Result<()>{
    let args = Args::parse();
    let size = args.lines;
    let path = args.path; 
    let chars = args.characters;

    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&path)?;

    println!("Generating file...");

    for _ in 0..size {
        let string = generate_string(chars);
        writeln!(file, "{}", string)?;
    }
    println!("File generated!");
    println!("Lines: {}", size);
    println!("Line size: {}", chars);

    Ok(())
}


/// Generate a random string of Alphanumeric characters.
///
/// # Arguments
///
/// * `length` - The length of the string. 
///
/// # Examples
///
/// ```
/// generate_string(8) // "fasdhlkk"
/// ```
fn generate_string(length: usize) -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    return s;
}
