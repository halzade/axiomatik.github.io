use crate::data::time::to_prague_time;
use chrono::{DateTime, Datelike, Utc, Weekday};
// TODO XX nejsou vyřešeny státní svátky

pub const CZECH_MONTHS_CAPITAL: [&str; 12] = [
    "Leden",
    "Únor",
    "Březen",
    "Duben",
    "Květen",
    "Červen",
    "Červenec",
    "Srpen",
    "Září",
    "Říjen",
    "Listopad",
    "Prosinec",
];

pub const CZECH_MONTHS: [&str; 12] = [
    "leden", "unor", "brezen", "duben", "kveten", "cerven", "cervenec", "srpen", "zari", "rijen",
    "listopad", "prosinec",
];

pub const CZECH_MONTHS_GENITIVE: [&str; 12] = [
    "ledna",
    "února",
    "března",
    "dubna",
    "května",
    "června",
    "července",
    "srpna",
    "září",
    "října",
    "listopadu",
    "prosince",
];

pub const fn get_czech_month(month: u32) -> &'static str {
    let idx = (month - 1) as usize;
    CZECH_MONTHS[idx]
}

#[allow(unused)]
const fn get_czech_month_capital(month: u32) -> &'static str {
    let idx = (month - 1) as usize;
    CZECH_MONTHS_CAPITAL[idx]
}

const fn get_czech_month_genitive(month: u32) -> &'static str {
    CZECH_MONTHS_GENITIVE[(month - 1) as usize]
}

pub fn day_of_week(utc: DateTime<Utc>) -> &'static str {
    let now = to_prague_time(utc);
    match now.weekday() {
        Weekday::Mon => "Pondělí",
        Weekday::Tue => "Úterý",
        Weekday::Wed => "Středa",
        Weekday::Thu => "Čtvrtek",
        Weekday::Fri => "Pátek",
        Weekday::Sat => "Sobota",
        Weekday::Sun => "Neděle",
    }
}

pub fn safe_article_file_name(title: &str) -> String {
    title
        .trim()
        .to_lowercase()
        .chars()
        .map(|c| match c {
            'a'..='z' | '0'..='9' => c,

            // samohlásky
            'á' => 'a',
            'é' => 'e',
            'ě' => 'e',
            'í' => 'i',
            'ó' => 'o',
            'ú' => 'u',
            'ů' => 'u',

            // souhlásky
            'č' => 'c',
            'ď' => 'd',
            'ň' => 'n',
            'ř' => 'r',
            'š' => 's',
            'ť' => 't',
            'ý' => 'y',
            'ž' => 'z',
            _ => '-',
        })
        .collect::<String>()
}

/**
 * Prague display date from UTC
 */
pub fn display_date(utc: DateTime<Utc>) -> String {
    let day_name = day_of_week(utc);
    let now = to_prague_time(utc);

    let month_name_genitive = get_czech_month_genitive(now.month());

    format!("{} {}. {} {}", day_name, now.day(), month_name_genitive, now.year())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_get_czech_month() {
        assert_eq!(get_czech_month(1), "leden");
        assert_eq!(get_czech_month(12), "prosinec");
    }

    #[test]
    fn test_get_czech_month_capital() {
        assert_eq!(get_czech_month_capital(1), "Leden");
        assert_eq!(get_czech_month_capital(12), "Prosinec");
    }

    #[test]
    fn test_get_czech_month_genitive() {
        assert_eq!(get_czech_month_genitive(1), "ledna");
        assert_eq!(get_czech_month_genitive(12), "prosince");
    }

    #[test]
    fn test_day_of_week() {
        let dt = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap(); // Monday
        assert_eq!(day_of_week(dt), "Pondělí");
        let dt = Utc.with_ymd_and_hms(2024, 1, 7, 12, 0, 0).unwrap(); // Sunday
        assert_eq!(day_of_week(dt), "Neděle");
    }

    #[test]
    fn test_save_article_file_name() {
        assert_eq!(
            safe_article_file_name(&"Příliš žluťoučký kůň".to_string()),
            "prilis-zlutoucky-kun"
        );
        assert_eq!(safe_article_file_name(&"Hello World!".to_string()), "hello-world-");
    }

    #[test]
    fn test_formatted_article_date() {
        let dt = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
        assert_eq!(display_date(dt), "Pondělí 1. ledna 2024");
    }
}
