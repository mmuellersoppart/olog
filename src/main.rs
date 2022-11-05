use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
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

fn connect_to_db(path: PathBuf) -> File {
    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(path)
        .expect("Could not connect to the database.")
}

fn load(f: &mut File) -> Result<Vec<loglib::Log>, Box<dyn Error>> {

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
    Ok(logs)

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
    let in_memory = load(&mut db).unwrap();



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

