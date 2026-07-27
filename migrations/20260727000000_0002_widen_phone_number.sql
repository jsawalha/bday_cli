-- phone_number was VARCHAR(10), too narrow for formatted numbers like 780-486-1050
ALTER TABLE birthdays ALTER COLUMN phone_number TYPE VARCHAR(20);
