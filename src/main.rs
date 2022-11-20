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

#[derive(Debug, PartialEq)]
enum CMD {
    W,  // write
    R,  // read
    A,  // add (without timer)
    Q,  // quit
}


impl CMD {
    fn to_value(&self) -> &'static str {
        match self {
            CMD::W => "W",
            CMD::R => "R",
            CMD::A => "A",
            CMD::Q => "Q"
        }
    }

    fn from_string(input: &str) -> Result<CMD, Error> {
        match input.to_uppercase().as_str() {
            "W" => Ok(CMD::W),
            "R" => Ok(CMD::R),
            "A" => Ok(CMD::A),
            "Q" => Ok(CMD::Q),
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

fn guess_next_log() -> (String, u8) {
    // displays a possible next log
    /*
    e.g.
    1  2    3      4   5   678
    Me,Olog,Coding,CLI,Add,,,,

    And the user can select the number where they'd like to
    begin editing.
     */

    // TODO: do some analysis to determine a likely next log
    let mut log = Log::new(None, None, Some("Me".to_string()), None, Some("Olog".to_string()), None, None, None, None, None, None, None);

    // Take guessed log and turn into record

    let mut wtr = WriterBuilder::new()
        .has_headers(false)
        .from_writer(vec![]);

    wtr.serialize(&log).expect("Failed to serialize guess");
    let record = String::from_utf8(wtr.into_inner().unwrap()).unwrap();

    // TODO: print log with numbers

    let mut options: String = String::new();
    let mut letter_pos: Vec<usize> = vec!();
    let mut letter: char = 'a';
    for (i, char) in record.chars().enumerate() {
        if char == ',' {
            options.push(letter);
            letter_pos.push(i);
            letter = char::from((letter as u8) + 1)
        } else {
            options.push(' ');
        }
    }

    println!("{}", options);
    println!("{}", record);

    // Get users input
    let mut input_buffer = String::new();
    let stdin = stdin(); // We get `Stdin` here.
    // TODO: validate
    stdin.read_line(&mut input_buffer).expect("Failed to read std input.");
    let first_byte = input_buffer.as_bytes().iter().next();
    let start_step = first_byte.unwrap() - ('a' as u8);

    // TODO: reduce record based on step
    let position = letter_pos[start_step as usize];
    let record = record[0..position].to_string();

    // let start_point = input_buffer.as_bytes()[0]

    (record, start_step)
}

fn parse_add_input(input: &str) {
    // determines the type of add
    unimplemented!();
}

fn add_log(input: &str) {
    /*
    You have to input everything at that point, even start and end time.
    > olog A "Olog, Planning, CLI interface, ..."
    > olog A -i  // interactive input
     */

    let (guess, start_point) = guess_next_log();

    let mut interative_log = String::from(guess);
    let interactive_order = [
        (0, "User_id"),
        (1, "Organization_id"),
        (2, "Organization"),
        (3, "Project_id"),
        (4, "Project"),
        (5, "Subproject_id"),
        (6, "Sub Project"),
        (7, "Activity"),
        (8, "Detail1"),
        (9, "Detail2"),
        (10, "Spirits"),
        (11, "Notes"),
    ];

    for log_part in interactive_order {
        match log_part.0 {
            x if x <= start_point => {},
            0 | 1 | 3 | 5  => interative_log.push(','),
            _ => {
                // Get input for line and guess each line
                println!("{}", log_part.1);
                interative_log.push_str("filled in,")
            }
        }
    }

    // TODO: add time
    println!("{}", interative_log)
    // unimplemented!();
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

        // parse command
        let stdin = stdin(); // We get `Stdin` here.
        stdin.read_line(&mut input_buffer).expect("Failed to read std input.");

        // first word of input
        let raw_command = input_buffer.split(" ").nth(0).unwrap();
        let raw_command = raw_command.trim();
        let command = match CMD::from_string(raw_command) {
            Ok(command) => command,
            Err(e) => {
                //TODO: list all possible commands. Or suggest --help
                println!("{:?}", e);
                continue
            },
        };

        // parse rest of the command and act on it
        match command {
            CMD::Q => break,
            CMD::A => add_log(&input_buffer),
            _ => unimplemented!(),
        }
        if command == CMD::Q {
            break
        }


        input_buffer.clear();

        println!("{:?}", command);


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

