mod cli;
mod entry;

use serde_json::Result;

fn main() -> Result<()> {
    let args = cli::parse();

    match &args.command {
        cli::Commands::Add {
            food_name: name,
            calories: quantity,
        } => {
            entry::add_to_daily_log(entry::Entry::new(name.to_string(), *quantity));
        }
        cli::Commands::View => {
            entry::view_daily_log();
        }
        cli::Commands::Total => {
            println!(
                "Today's caloric total: {}",
                entry::total_calories_for_daily_log()
            )
        }
    }

    Ok(())
}
