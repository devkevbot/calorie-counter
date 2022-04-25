use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    fs::{File, OpenOptions},
    io::BufReader,
};

pub fn add_to_daily_log(entry: Entry) {
    let mut daily_log = get_daily_log();
    daily_log.push(entry);
    overwrite_daily_log(daily_log);
}

pub fn view_daily_log() {
    let daily_log = get_daily_log();
    let printed_str = match &daily_log.entries.len() {
        0 => "No entries for today!".to_string(),
        _ => format!("{}", &daily_log),
    };
    println!("{printed_str}")
}

pub fn total_calories_for_daily_log() -> u16 {
    get_daily_log().total_calories()
}

fn get_daily_log() -> DailyLog {
    let daily_log_file = open_daily_log_file();
    let reader = BufReader::new(daily_log_file);
    let daily_log: DailyLog = serde_json::from_reader(reader).unwrap_or(DailyLog::new());
    daily_log
}

fn overwrite_daily_log(daily_log: DailyLog) {
    let daily_log_file = open_daily_log_file();
    serde_json::to_writer_pretty(&daily_log_file, &daily_log).unwrap();
}

fn open_daily_log_file() -> File {
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

    pub fn total_calories(&self) -> u16 {
        let calories_values = self
            .entries
            .iter()
            .fold(0, |total_calories, entry| total_calories + entry.calories);
        calories_values
    }
}

impl Display for DailyLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            &self
                .entries
                .iter()
                .map(|entry| entry.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
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

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} calories", self.food_name, self.calories)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_calories_is_correct() {
        let mut dl = DailyLog::new();
        let e1 = Entry::new("Apples".to_string(), 100);
        let e2 = Entry::new("Cake".to_string(), 450);

        dl.push(e1);
        dl.push(e2);

        assert_eq!(dl.total_calories(), 550);
    }
}
