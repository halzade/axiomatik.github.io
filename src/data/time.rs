use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use chrono_tz::Tz::Europe__Prague;

const PRAGUE: Tz = Europe__Prague;

pub fn to_prague_time(utc: DateTime<Utc>) -> DateTime<Tz> {
    utc.with_timezone(&PRAGUE)
}
