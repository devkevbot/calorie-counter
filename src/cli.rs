use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

pub fn parse() -> Cli {
    Cli::parse()
}

#[derive(Subcommand)]
pub enum Commands {
    /// Adds a new entry to the daily log
    Add {
        /// The name of the food
        #[clap(short, long)]
        food_name: String,

        /// How many calories were consumed
        #[clap(short, long)]
        calories: u16,
    },
    /// View the contents of the daily log
    View,
}
