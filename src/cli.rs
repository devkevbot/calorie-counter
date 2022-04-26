use crate::entry;
use clap::{Parser, Subcommand};

pub fn run() {
    let args = Cli::parse();
    execute_command(&args);
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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

fn execute_command(args: &Cli) {
    let mut ctx = entry::Context::new();

    match &args.command {
        Commands::Add {
            food_name: name,
            calories: quantity,
        } => {
            let entry = entry::Entry::new(name.to_string(), *quantity);
            entry::add::execute(&mut ctx, entry)
        }
        Commands::View => entry::view::execute(ctx),
    };
}
