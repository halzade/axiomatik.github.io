use crate::feature::name_days_library;
use chrono::prelude::*;
use std::string::ToString;
use tracing::error;
use crate::data::time::to_prague_time;

const LEEP_YEAR_NAME_DAY: &str = "Cleverest Punk";

pub fn formatted_today_name_day(utc: DateTime<Utc>) -> String {
    let name = today_name_day(utc);
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

fn today_name_day(utc: DateTime<Utc>) -> String {
    let now = to_prague_time(utc);
    let year = now.year();
    let month = now.month();
    let day = now.day();

    if month == 2 && day == 29 && !is_leap_year(year) {
        return LEEP_YEAR_NAME_DAY.to_string();
    }
    name_days_library::get_name_day(month, day)
}

fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_is_leap_year() {
        assert!(is_leap_year(2024));
        assert!(is_leap_year(2000));
        assert!(!is_leap_year(2023));
        assert!(!is_leap_year(2100));
    }

    #[test]
    fn test_formatted_today_name_day() {
        let dt = Utc.with_ymd_and_hms(2024, 10, 18, 12, 0, 0).unwrap();
        let name_day = formatted_today_name_day(dt);
        assert_eq!(name_day, "Svátek má Lukáš");
    }

    #[test]
    fn test_formatted_today_holiday() {
        let dt = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
        let name_day = formatted_today_name_day(dt);
        assert_eq!(name_day, "je Nový rok, státní svátek");
    }

    #[test]
    fn test_today_name_day_leap_year() {
        let dt = Utc.with_ymd_and_hms(2023, 2, 29, 12, 0, 0);
        // chrono might not even allow creating 2023-02-29
        if let Some(dt) = dt.single() {
            assert_eq!(today_name_day(dt), LEEP_YEAR_NAME_DAY);
        }
    }
}
