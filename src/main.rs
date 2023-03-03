use std::{
    fs, 
    io::Write, 
    time::Instant
};
use clap::Parser;
use anyhow::Result;
use colorful::{Color, Colorful};
use spinners::{Spinner, Spinners};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of file to generate or append.
    #[arg()]
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

    let msg_start = "Generating file...".color(Color::Green);
    let mut sp = Spinner::new(Spinners::Moon, msg_start.to_string());

    let start = Instant::now();
    for _ in 0..size {
        let string = gf::generate_string(chars);
        writeln!(file, "{}", string)?;
    }
    let end = start.elapsed();

    let msg_finished = format!("File generated in {:?}", end).color(Color::Green);
    sp.stop_and_persist("✔", msg_finished.to_string());

    println!("{:<5} | {:<5}", "Lines", "Chars/line");
    println!("{:<5} | {:<5}", size, chars);

    Ok(())
}

