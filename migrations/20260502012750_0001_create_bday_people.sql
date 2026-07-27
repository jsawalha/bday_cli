-- Add migration script here
CREATE TABLE IF NOT EXISTS birthdays (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    birth_date DATE NOT NULL,
    phone_number VARCHAR(10)
);