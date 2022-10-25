
use clap::Parser;
use std::path::{Path, PathBuf};

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


}

