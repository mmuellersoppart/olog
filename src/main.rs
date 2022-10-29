use clap::Parser;
use std::path::{Path, PathBuf};
use std::mem;

/// CLI for Open Log (olog)

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long)]
    password: Option<String>,
    
    #[arg(short, long, value_name = "PATH")]
    store: Option<PathBuf>
}

fn main() {

    let args = Args::parse();

    // At startup, you the user declares your name, password, and store location
    if let name = args.name {
        println!("{}", name)
    }

    let mut logs: Vec<loglib::Log> = vec!();
    let log = loglib::Log::new(
        None,
        None,
        Some(String::from("Olog")),
        None,
        Some(String::from("CLI")),
        None,
        None,
        Some(String::from("Coding")),
        Some(String::from("Adding Entry")),
        None,
        Some(8),
        Some(String::from("Not much mention. Building the types so the fake cli has something to work with.")),
    );

    println!("{:?}", log);
    println!("{}", mem::size_of_val(&log));
}

