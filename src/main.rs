use std::{
    fs, 
    io::Write, 
    time::Instant
};
use clap::Parser;
use anyhow::Result;
use colorful::{Color, Colorful};

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

    println!("{}", "Generating file...".color(Color::Blue));
    let start = Instant::now();
    for _ in 0..size {
        let string = generate_file::generate_string(chars);
        writeln!(file, "{}", string)?;
    }
    let end = start.elapsed();

    println!("{} {:?}", 
        "File Generated in".color(Color::Green), 
        end
    );
    println!("{:<5} | {:<5}", "Lines", "Chars/line");
    println!("{:<5} | {:<5}", size, chars);

    Ok(())
}

