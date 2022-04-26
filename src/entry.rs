use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    fs::{File, OpenOptions},
    io::BufReader,
};

pub mod add {
    use super::*;

    pub fn execute(entry: Entry) -> String {
        today::append_entry(entry);
        format_output_messsage()
    }

    fn format_output_messsage() -> String {
        String::from("Successfully added entry to today's log!")
    }
}

pub mod view {
    use super::*;

    pub fn execute() -> String {
        let daily_log = today::read_as_log();
        format_output_messsage(&daily_log)
    }

    fn format_output_messsage(daily_log: &DailyLog) -> String {
        let formatted_message = match &daily_log.entries.len() {
            0 => "No entries for today!".to_string(),
            _ => format!("Today's log\n{}", &daily_log),
        };
        formatted_message
    }
}

pub mod total {
    use super::*;

    pub fn execute() -> String {
        let daily_log = today::read_as_log();
        format_output_messsage(&daily_log)
    }

    fn format_output_messsage(daily_log: &DailyLog) -> String {
        let total_calories = daily_log.total_calories();
        let formatted_message = match total_calories {
            0 => "No calories recorded for today!".to_string(),
            _ => format!("Today's caloric total: {}", total_calories),
        };
        formatted_message
    }
}

mod today {
    use super::*;

    pub fn read_as_log() -> DailyLog {
        let file = get_stored_file();
        let reader = BufReader::new(file);
        let daily_log: DailyLog = serde_json::from_reader(reader).unwrap_or(DailyLog::new());
        daily_log
    }

    fn get_stored_file() -> File {
        OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(get_path())
            .unwrap()
    }

    fn get_path() -> String {
        format!("{}.json", Local::now().format("%Y-%m-%d").to_string())
    }

    pub fn append_entry(entry: Entry) {
        let mut daily_log = read_as_log();
        daily_log.append(entry);
        let file = get_stored_file();
        serde_json::to_writer_pretty(&file, &daily_log).unwrap();
    }
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

    pub fn append(&mut self, entry: Entry) {
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

        dl.append(e1);
        dl.append(e2);

        assert_eq!(dl.total_calories(), 550);
    }
}
