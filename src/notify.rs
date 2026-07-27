use std::process::Command;

use crate::models::Person;

/// Fires a desktop notification via libnotify (notify-send ships with Pop!_OS).
/// Returns an error if the binary is missing or exits non-zero, so the caller
/// can fall back to stdout rather than failing silently under cron.
pub fn send_desktop_notification(title: &str, body: &str) -> Result<(), String> {
    let status = Command::new("notify-send")
        // Birthday reminders should stay on screen until dismissed.
        .arg("--urgency=critical")
        .arg("--app-name=bdaycli")
        .arg(title)
        .arg(body)
        .status()
        .map_err(|e| format!("could not run notify-send: {e}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("notify-send exited with {status}"))
    }
}

/// Builds the notification body: one line per person, with the number to text.
pub fn birthday_message(people: &[&Person]) -> String {
    people
        .iter()
        .map(|p| {
            let phone = p.phone_number.as_deref().unwrap_or("no phone");
            format!("{} turns {} today — {}", p.name, p.get_age(), phone)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Same lines, minus phone numbers. ntfy topics on the public server are
/// readable by anyone who knows the topic name, so the push deliberately
/// carries less than the local desktop notification does.
pub fn birthday_message_no_phone(people: &[&Person]) -> String {
    people
        .iter()
        .map(|p| format!("{} turns {} today", p.name, p.get_age()))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Pushes to ntfy.sh so the reminder reaches the phone, not just this desktop.
/// No-op (Ok) when NTFY_TOPIC isn't set, so the CLI still works without it.
pub async fn send_push(title: &str, body: &str) -> Result<(), String> {
    let topic = match std::env::var("NTFY_TOPIC") {
        Ok(t) if !t.trim().is_empty() => t,
        _ => return Ok(()),
    };
    let server = std::env::var("NTFY_SERVER").unwrap_or_else(|_| "https://ntfy.sh".to_string());

    let response = reqwest::Client::new()
        .post(format!("{server}/{topic}"))
        .header("Title", title)
        .header("Tags", "birthday")
        .header("Priority", "high")
        .body(body.to_string())
        .send()
        .await
        .map_err(|e| format!("could not reach {server}: {e}"))?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("ntfy returned {}", response.status()))
    }
}
