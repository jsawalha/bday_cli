use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version)]
#[command(about = "CLI tool to remember birthdays of people you know and send them messages!")]
pub struct Cli {
    #[clap(subcommand)]
    pub actions: ActionArgs,
}

#[derive(Debug, Subcommand)]
pub enum ActionArgs {
    /// Add birthday person!
    Add{
        name: String,
        birth_date: String,
        phone_number: Option<String>
    },
    /// Delete birthday person
    Delete{name: String},
    /// Get list of birthday people
    List,
    /// Get Today's Date and any birthdays
    Today{},
    /// Check if it's someones birthday today that has a phone number
    Notify,
    /// Search for someone's birthday based on name, year, month, day
    /// Filters are named so any one of them can be used on its own.
    Search {
        /// Name (partial match, case-insensitive)
        #[arg(short, long)]
        name: Option<String>,
        /// Year
        #[arg(short, long)]
        year: Option<i32>,
        /// Month
        #[arg(short, long)]
        month: Option<u32>,
        /// Day
        #[arg(short, long)]
        day: Option<u32>,
        /// Phone number
        #[arg(short, long)]
        phone_number: Option<String>
    },
}
