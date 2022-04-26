use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::{fmt::Display, fmt::Formatter, io::BufReader};

pub mod add {
    use super::*;

    pub fn execute(ctx: &mut Context, entry: Entry) {
        ctx.rw.write_log(entry);
        done_message()
    }

    fn done_message() {
        println!("Successfully added entry!");
    }
}

pub mod view {
    use super::*;

    pub fn execute(ctx: Context) {
        let log = ctx.rw.read_log();
        println!("{}", log);
    }
}

pub struct Context {
    rw: log::ReaderWriter,
}

impl Context {
    pub fn new() -> Self {
        Self {
            rw: log::ReaderWriter::new(),
        }
    }
}

pub mod log {
    use super::*;

    pub struct ReaderWriter {
        log: Log,
        file: File,
    }

    impl ReaderWriter {
        pub fn new() -> Self {
            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .open(format!(
                    "{}.json",
                    Local::now().format("%Y-%m-%d").to_string()
                ))
                .unwrap();
            let reader = BufReader::new(&file);
            let log: Log = serde_json::from_reader(reader).unwrap_or(Log::new());
            Self { log, file }
        }

        pub fn read_log(&self) -> &Log {
            &self.log
        }

        pub fn write_log(&mut self, entry: Entry) {
            self.log.append(entry);
            serde_json::to_writer_pretty(&self.file, &self.log).unwrap();
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct Log {
        entries: Vec<Entry>,
    }

    impl Log {
        pub fn new() -> Self {
            Self {
                entries: Vec::new(),
            }
        }

        pub fn append(&mut self, entry: Entry) {
            self.entries.push(entry);
        }
    }

    impl Display for Log {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match &self.entries.len() {
                0 => write!(f, "<No entries>"),
                _ => write!(
                    f,
                    "{}",
                    &self
                        .entries
                        .iter()
                        .map(|entry| entry.to_string())
                        .collect::<Vec<String>>()
                        .join("\n")
                ),
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Entry {
    food_name: String,
    calories: u16,
}

impl Entry {
    pub fn new(food_name: String, calories: u16) -> Self {
        Self {
            food_name,
            calories,
        }
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} calories", self.food_name, self.calories)
    }
}
