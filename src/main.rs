use std::fs::{File, OpenOptions};
use std::io::{Read, Write, stdin, ErrorKind};
use std::mem;
use std::path::{Path, PathBuf};

use clap::Parser;
use csv::{Reader, Writer, WriterBuilder};

use loglib::Log;

/// CLI for Open Log (olog)

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long)]
    password: Option<String>,
    
    #[arg(short, long, value_name = "PATH")]
    db: Option<String>
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("The command submitted cannot be parsed.")]
    InvalidCommand,
}

enum CMD {
    W,
    R,
    A,
}

impl CMD {
    fn to_value(&self) -> &'static str {
        match self {
            CMD::W => "work",
            CMD::R => "rest",
            CMD::A => "add",
        }
    }

    fn from_string(input: &str) -> Result<CMD, Error> {
        match input.to_lowercase().as_str() {
            "work" => Ok(CMD::W),
            "read" => Ok(CMD::R),
            "add" => Ok(CMD::A),
            _ => Err(Error::InvalidCommand)
        }
    }
}

fn connect_to_db(path: PathBuf) -> File {
    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(path)
        .expect("Could not connect to the database.")
}

fn load(f: &mut File) -> Vec<loglib::Log> {

    // TODO: read csv and deserialize to logs
    let mut rdr = csv::Reader::from_reader(f);
    let mut logs = vec!();
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        match result {
            Err(e) => {},
            Ok(t) => logs.push(t)
        }
    }
    logs
}

fn main() {

    let args = Args::parse();

    // At startup, you the user declares your name, password, and store location
    if let name = args.name {
        println!("{}", name)
    }

    // Retrieve or create db
    let path = args.db.unwrap_or("./db.txt".to_string());
    println!("Connecting to the db at {}", path);
    let mut db= connect_to_db(PathBuf::from(path));
    let in_memory = load(&mut db);

    let mut input_buffer = String::new();
    loop {
        println!("I want your input");
        let stdin = stdin(); // We get `Stdin` here.
        stdin.read_line(&mut input_buffer).expect("Failed to read std input.");

        if input_buffer == "hi\n" {
            break
        }
    }


    // do log
    let log = Log::new(
        None,
        None,
        Some("Olog".to_string()),
        None,
        Some("CLI".to_string()),
        None,
        None,
        Some("Coding".to_string()),
        Some("Adding Entry".to_string()),
        None,
        Some(8),
        Some("Not much mention. Building the types so the fake cli has something to work with.".to_string()),
    );

    let has_headers = if in_memory.len() == 0 {
        true
    } else {
        false
    } ;

    let mut wtr = WriterBuilder::new()
        .has_headers(has_headers)
        .from_writer(db);

    // When writing records with Serde using structs, the header row is written
    // automatically.
    wtr.serialize(log).expect("Unable to serialize log.");
    wtr.flush().unwrap();


    // println!("{}", log);
    //
    // in_memory.push(log.to_string());
}

