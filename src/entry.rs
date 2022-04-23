use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::BufReader;

pub fn add_to_daily_log(entry: Entry) {
    let mut daily_log = get_daily_log();
    daily_log.push(entry);
    overwrite_daily_log(daily_log);
}

pub fn view_daily_log() {
    let daily_log = get_daily_log();
    println!(
        "{}",
        serde_json::to_string_pretty(&daily_log.entries).unwrap()
    )
}

fn get_daily_log() -> DailyLog {
    let daily_log_file = daily_log_file();
    let reader = BufReader::new(daily_log_file);
    let daily_log: DailyLog = serde_json::from_reader(reader).unwrap_or(DailyLog::new());
    daily_log
}

fn overwrite_daily_log(daily_log: DailyLog) {
    let daily_log_file = daily_log_file();
    serde_json::to_writer_pretty(&daily_log_file, &daily_log).unwrap();
}

fn daily_log_file() -> File {
    OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(path_to_daily_log_file())
        .unwrap()
}

fn path_to_daily_log_file() -> String {
    format!("{}.json", Local::now().format("%Y-%m-%d").to_string())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DailyLog {
    entries: Vec<Entry>,
}

impl DailyLog {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn push(&mut self, entry: Entry) {
        self.entries.push(entry);
    }
}

#[derive(Serialize, Deserialize, Debug)]
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
