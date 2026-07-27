# bdaycli

A command-line birthday tracker written in Rust. Stores birthdays in Postgres, and on the morning of someone's birthday it sends you a desktop notification and a push notification to your phone — so you can send the text yourself.

Built as a project for learning Rust: clap for the CLI, SQLx for compile-time-checked SQL, chrono for date math, and tokio for async.

## Features

- Add, delete, list, and search birthdays
- Age and days-until-birthday calculated automatically
- Partial, case-insensitive name search; filter by year, month, day, or phone number
- `notify` command designed to run from cron: silent when there's nothing to report
- Two delivery channels — a local desktop notification and a push to your phone via [ntfy.sh](https://ntfy.sh)

## Requirements

- Rust (2021 edition)
- PostgreSQL
- `notify-send` for desktop notifications (preinstalled on most Linux desktops)
- Optional: the [ntfy](https://ntfy.sh) app on your phone for push notifications

## Setup

Create a `.env` file in the project root:

```
DATABASE_URL=postgres://user:password@localhost:5432/birthdays
NTFY_TOPIC=your_unguessable_topic_name
```

Then create the database and build:

```bash
createdb birthdays
cargo build --release
```

Migrations run automatically on startup, so the table is created on first use.

> **Note:** SQLx verifies every query against the real schema at compile time, so **Postgres must be running to build the project**, not just to run it. If you see `error communicating with database` from `cargo build`, that's why.

## Usage

```bash
# Add someone (phone number optional)
bdaycli add Maria 1985-07-27 555-0199
bdaycli add Sam 2001-12-01

# List everyone
bdaycli list

# Whose birthday is today, plus who's next
bdaycli today

# Search — any filter works on its own
bdaycli search --name ar
bdaycli search --month 7
bdaycli search --year 1990 --month 3

# Remove someone
bdaycli delete Sam

# Send notifications for today's birthdays (silent if there are none)
bdaycli notify
```

Dates must be in `YYYY-MM-DD` format.

## Scheduling

Run `notify` every morning with cron (`crontab -e`):

```
0 9 * * * cd /path/to/bdaycli && ./target/release/bdaycli notify
```

The `cd` matters — the `.env` file is read relative to the working directory.

If the desktop notification doesn't appear when run from cron, add your session bus address to the crontab line:

```
DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/1000/bus
```

Cron skips missed runs, so if the machine is asleep at 9am nothing is sent. A systemd user timer with `Persistent=true` will catch up on wake instead.

## Push notifications

Install the ntfy app, subscribe to a topic, and put that topic in `NTFY_TOPIC`. If it isn't set, push is skipped and everything else still works.

**Choose an unguessable topic name.** Topics on the public ntfy server have no authentication — anyone who knows or guesses the name receives everything published to it. For that reason the push payload deliberately contains names and ages only; phone numbers stay in the local desktop notification.

## Project structure

```
src/
  main.rs     Wires CLI subcommands to database calls
  cli.rs      Subcommand definitions (clap)
  db.rs       Database queries (SQLx)
  models.rs   Person struct, age and birthday math
  notify.rs   Desktop notifications and ntfy push
migrations/   SQL schema, applied automatically at startup
```

## Tests

```bash
cargo test
```

Covers the date logic in `models.rs` — age calculation and days-until-birthday, including the year-rollover case where a birthday has already passed.

## Limitations

- Postgres must be running to build and to run
- Notifications only fire if the machine is on when cron triggers
- Sending the actual text is manual by design — iOS has no public API for a third-party program to send SMS from your number unattended
