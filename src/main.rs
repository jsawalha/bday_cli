pub mod models;
pub mod db;
pub mod cli;
pub mod notify;

use clap::Parser;
use chrono::NaiveDate;

use crate::db::{add_person, connect, delete_person, get_all_persons, search_persons, get_persons_with_phone};
use crate::cli::{Cli, ActionArgs};
use crate::notify::{birthday_message, birthday_message_no_phone, send_desktop_notification, send_push};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let pool = connect().await;
    let client = Cli::parse();

    // 3. Match against the CLI subcommands
    match client.actions {
        ActionArgs::Add { name, birth_date, phone_number }  => {
            // clap hands us the date as a String, add_person needs a NaiveDate.
            let birth_date = match NaiveDate::parse_from_str(&birth_date, "%Y-%m-%d") {
                Ok(date) => date,
                Err(_) => {
                    eprintln!("'{birth_date}' isn't a valid date. Use YYYY-MM-DD, e.g. 1990-03-15.");
                    std::process::exit(1);
                }
            };
            add_person(&pool, name.clone(), birth_date, phone_number).await?;
            println!("Added {name} ({birth_date}).");
        }
        ActionArgs::Delete { name } => {
            let removed = delete_person(&pool, &name).await?;
            if removed == 0 {
                println!("Nobody named '{name}' in the list.");
            } else {
                println!("Removed {removed} entry/entries for '{name}'.");
            }
        }
        ActionArgs::List => {
            let people = get_all_persons(&pool).await?;
            if people.is_empty() {
                println!("No birthdays saved yet. Add one with: bdaycli add <name> <YYYY-MM-DD>");
            } else {
                println!("{} birthday(s):", people.len());
                for person in &people {
                    println!("  {}", person.summary());
                }
            }
        }
        ActionArgs::Notify => {
            let people_w_phone = get_persons_with_phone(&pool).await?;
            let today_bday_w_phone: Vec<_> = people_w_phone
                .iter()
                .filter(|p| p.days_until_birthday() == 0)
                .collect();
            

            if !today_bday_w_phone.is_empty() {
                let title = if today_bday_w_phone.len() == 1 {
                    "Birthday today!".to_string()
                } else {
                    format!("{} birthdays today!", today_bday_w_phone.len())
                };
                let body = birthday_message(&today_bday_w_phone);

                // Two independent channels: the desktop notification is local and
                // carries phone numbers, the push reaches the phone and doesn't.
                // Neither failure should stop the other from being tried.
                let desktop = send_desktop_notification(&title, &body);
                let push = send_push(&title, &birthday_message_no_phone(&today_bday_w_phone)).await;

                if let Err(e) = &desktop {
                    eprintln!("desktop notification failed: {e}");
                }
                if let Err(e) = &push {
                    eprintln!("push failed: {e}");
                }
                // If both channels failed, make sure the reminder still lands
                // somewhere instead of vanishing (e.g. under cron).
                if desktop.is_err() && push.is_err() {
                    println!("{title}\n{body}");
                }
            }
        }
        ActionArgs::Today {} => {
            let people = get_all_persons(&pool).await?;
            let todays: Vec<_> = people
                .iter()
                .filter(|p| p.days_until_birthday() == 0)
                .collect();

            if todays.is_empty() {
                println!("No birthdays today.");
                // Still useful to know what's coming up next.
                if let Some(next) = people.iter().min_by_key(|p| p.days_until_birthday()) {
                    println!("Next up: {}", next.summary());
                }
            } else {
                for person in todays {
                    println!("Birthday today: {}", person.summary());
                }
            }
        }
        ActionArgs::Search { name, year, month, day, phone_number } => {
            let found = search_persons(&pool, name, year, month, day, phone_number).await?;
            if found.is_empty() {
                println!("No matches.");
            } else {
                println!("{} match(es):", found.len());
                for person in &found {
                    println!("  {}", person.summary());
                }
            }
        }
    }

    Ok(())
}
