use chrono::{Datelike, NaiveDate};

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) {
        None
    } else {
        Some(
            NaiveDate::from_yo_opt(year as i32, 183).unwrap().weekday()
        )
    }
}
