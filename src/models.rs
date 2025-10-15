use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Transaction {
    pub id: Option<i64>,
    pub date: String,
    pub person: String,
    pub amount: i64,
    pub note: Option<String>,
}

impl Transaction {
    pub fn new(
        date: String,
        person: String,
        amount: i64,
        note: Option<String>,
    ) -> Result<Self, String> {
        if person.trim().is_empty() {
            return Err("Person name cannot be empty.".to_string());
        }

        if NaiveDate::parse_from_str(&date, "%Y-%m-%d").is_err() {
            return Err("Date must be in YYYY-MM-DD format.".to_string());
        }

        Ok(Transaction {
            id: None,
            date,
            person,
            amount,
            note,
        })
    }

    pub fn new_with_defaults(
        date: Option<String>,
        person: String,
        amount: i64,
        note: Option<String>,
    ) -> Result<Self, String> {
        let final_date = match date {
            Some(d) => d,
            None => Local::now().format("%Y-%m-%d").to_string(),
        };
        Self::new(final_date, person, amount, note)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Balance {
    pub person: String,
    pub total_amount: i64,
}
