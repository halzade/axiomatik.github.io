use crate::library_name_days;
use chrono::prelude::*;
use std::string::ToString;

const LEEP_YEAR_NAME_DAY: &'static str = "Cleverest Punk";

pub fn today_name_day() -> String {
    let now = Local::now();
    let year = now.year();
    let month = now.month();
    let day = now.day();
    if month == 2 && day == 29 && !is_leap_year(year) {
        return LEEP_YEAR_NAME_DAY.to_string();
    }
    library_name_days::get_nameday(month, day)
}

fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}
