use sqlx::{postgres::PgPoolOptions, PgPool}; // postgres operations
use std::env;
use crate::models::Person;

pub async fn connect() -> PgPool {
    // db_url, this will connect our backend will connect the backend with the database.
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set properly...");
    // let pool be a way to structure and a trial to connect to the database
    let pool = PgPoolOptions::new()
        .connect(&db_url)
        .await
        .expect("Failed to connect to DB");
    // this command will go into migrations 00001_create_users.sql and run the sql command! MAKES THE TABLE
    // migrate! is actually a macro that automatically connects to migrations folder
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    return pool;
}

pub async fn add_person(
    state: &PgPool,
    name: String,
    birth_date: chrono::NaiveDate,
    phone_number: Option<String>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO birthdays (name, birth_date, phone_number) VALUES ($1, $2, $3)",
        name,
        birth_date,
        phone_number
    )
    .execute(state)
    .await?;
    Ok(())
}


pub async fn get_all_persons( state: &PgPool) -> Result<Vec<Person>, sqlx::Error> {
    let get_all = sqlx::query_as!(Person, "SELECT * FROM birthdays ORDER BY name").fetch_all(state).await?;
    Ok(get_all)
}

pub async fn get_persons_with_phone( state: &PgPool) -> Result<Vec<Person>, sqlx::Error> {
    let get_numbers = sqlx::query_as!(Person, "SELECT * FROM birthdays WHERE phone_number is not null ORDER BY name").fetch_all(state).await?;
    Ok(get_numbers)
}

/// Deletes everyone whose name matches exactly. Returns how many rows went away
/// so the caller can tell "deleted" apart from "nobody by that name".
pub async fn delete_person(state: &PgPool, name: &str) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM birthdays WHERE name = $1", name)
        .execute(state)
        .await?;
    Ok(result.rows_affected())
}

/// Every filter is optional. Each `$n IS NULL OR ...` pair means "skip this
/// filter when the caller didn't supply it", which keeps one static query that
/// query_as! can still verify at compile time.
pub async fn search_persons(
    state: &PgPool,
    name: Option<String>,
    year: Option<i32>,
    month: Option<u32>,
    day: Option<u32>,
    phone_number: Option<String>,
) -> Result<Vec<Person>, sqlx::Error> {
    // Postgres date parts come back as f64/i32, so match the types it expects.
    let month = month.map(|m| m as f64);
    let day = day.map(|d| d as f64);
    let year = year.map(|y| y as f64);
    // Wrap the name in % so a partial name matches, case-insensitively.
    let name = name.map(|n| format!("%{n}%"));

    let found = sqlx::query_as!(
        Person,
        r#"
        SELECT * FROM birthdays
        WHERE ($1::text IS NULL OR name ILIKE $1)
          AND ($2::float8 IS NULL OR EXTRACT(YEAR  FROM birth_date) = $2)
          AND ($3::float8 IS NULL OR EXTRACT(MONTH FROM birth_date) = $3)
          AND ($4::float8 IS NULL OR EXTRACT(DAY   FROM birth_date) = $4)
          AND ($5::text IS NULL OR phone_number = $5)
        ORDER BY name
        "#,
        name,
        year,
        month,
        day,
        phone_number
    )
    .fetch_all(state)
    .await?;
    Ok(found)
}