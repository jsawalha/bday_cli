use sqlx::FromRow;
use chrono::{Datelike, Local, NaiveDate};

#[derive(Debug, FromRow)]
pub struct Person {
    pub id: i64,
    pub name: String,
    pub birth_date: chrono::NaiveDate,
    pub phone_number: Option<String>
}


impl Person {
    pub fn print_struct(&self) {
        println!("{:?}", self);
    }

    /// One-line summary used by the list/today/search output.
    pub fn summary(&self) -> String {
        let phone = self.phone_number.as_deref().unwrap_or("no phone");
        let days = self.days_until_birthday();
        // On the birthday itself get_age() already counts the new year,
        // so only look ahead when the birthday is still upcoming.
        let (turns, when) = match days {
            0 => (self.get_age(), "TODAY!".to_string()),
            1 => (self.get_age() + 1, "tomorrow".to_string()),
            n => (self.get_age() + 1, format!("in {n} days")),
        };
        format!(
            "{} — born {} (turns {}, {}) — {}",
            self.name, self.birth_date, turns, when, phone
        )
    }

    pub fn get_age(&self) -> i32 {
        let b_day = &self.birth_date;
        let today =  chrono::Local::now().date_naive();
        
        let mut age = (today.year() - b_day.year()) as i32;

        if today.month() < b_day.month() || (today.month() == b_day.month() && today.day() < b_day.day()) {
            age -= 1;
        }

        age

    }

    pub fn days_until_birthday(&self) -> i32 {
        // Get today's date
        let today = Local::now().date_naive();
        
        // 1. Try to set the birthday to THIS year
        let birthday_this_year = self.birth_date.with_year(today.year())
            .unwrap_or_else(|| NaiveDate::from_ymd_opt(today.year(), 3, 1).unwrap());
            
        // 2. Figure out the target birthday date
        let target_birthday = if birthday_this_year >= today {
            birthday_this_year // Birthday is today or later this year
        } else {
            // Birthday passed! Set it to NEXT year
            self.birth_date.with_year(today.year() + 1)
                .unwrap_or_else(|| NaiveDate::from_ymd_opt(today.year() + 1, 3, 1).unwrap())
        };
        
        // 3. Subtract the dates to get the remaining days
        let duration = target_birthday - today;
        duration.num_days() as i32
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn make_person() -> Person {
        Person {
        id: 1,
        name: "Tony".to_string(),
        birth_date: NaiveDate::from_ymd_opt(1990, 3, 15).unwrap(),
        phone_number: Some("780-486-1050".to_string())
        }
    }



    #[test]
    fn test_name() {
        let name_1 = make_person();
        assert_eq!("Tony".to_string(), name_1.name)
        }
    
    #[test]
    fn test_age() {
        let age_1 = make_person();
        assert_eq!(36, age_1.get_age())
    }

    #[test]
    fn test_days_until_bday() {
        let today = Local::now().date_naive();
        // 1. Create the person once so you are testing the same data
        let person = make_person(); 
        
        // 2. Use the if/else assignment syntax to assign the value correctly
        let expected_bday = if person.birth_date.with_year(today.year()).unwrap() < today {
            person.birth_date.with_year(today.year() + 1).unwrap()
        } else {
            person.birth_date.with_year(today.year()).unwrap()
        };

        // 3. Perform the math
        let duration = expected_bday - today;
        
        // 4. Assert against the function output
        assert_eq!(duration.num_days() as i32, person.days_until_birthday());
    }
        
    }
