use crate::library_name_days;
use chrono::prelude::*;
use std::string::ToString;
use tracing::error;

const LEEP_YEAR_NAME_DAY: &'static str = "Cleverest Punk";

pub fn formatted_today_name_day(now: DateTime<Local>) -> String {
    let name = today_name_day(now);
    if name.is_empty() {
        error!("empty name day");
        "".to_string()
    } else {
        if name.contains("_") {
            // dny, kdy svátek není
            return name.replace("_", "");
        }

        // normální svátek
        format!("Svátek má {}", name)
    }
}

fn today_name_day(now: DateTime<Local>) -> String {
    let year = now.year();
    let month = now.month();
    let day = now.day();

    if month == 2 && day == 29 && !is_leap_year(year) {
        return LEEP_YEAR_NAME_DAY.to_string();
    }
    library_name_days::get_name_day(month, day)
}

fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}
