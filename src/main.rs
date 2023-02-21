use std::{fs, io::Write};
use clap::Parser;
use anyhow::Result;
use random_string::generate;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Path of file to generate or append.
   #[arg(short, long)]
   path: String,

   /// Number of lines to generate.
   #[arg(short, long, default_value_t = 1000)]
   lines: usize,
}

fn main() -> Result<()>{
    let args = Args::parse();
    let size = args.lines;
    let path = args.path; 
    let charset = "abcdefghijklmnopqrstuvwxyz";

    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)?;

    for _ in 0..size {
        let string = generate(8, charset);
        writeln!(file, "{}", string)?;
    }

    Ok(())
}
