use chrono::{DateTime, Datelike, Local, Weekday};
use std::fs;

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

pub fn get_czech_month(month: u32) -> &'static str {
    let idx = (month - 1) as usize;
    CZECH_MONTHS[idx]
}

fn get_czech_month_capital(month: u32) -> &'static str {
    let idx = (month - 1) as usize;
    CZECH_MONTHS_CAPITAL[idx]
}

fn get_czech_month_genitive(month: u32) -> &'static str {
    CZECH_MONTHS_GENITIVE[(month - 1) as usize]
}

pub fn day_of_week(dtl: DateTime<Local>) -> &'static str {
    match dtl.weekday() {
        Weekday::Mon => "Pondělí",
        Weekday::Tue => "Úterý",
        Weekday::Wed => "Středa",
        Weekday::Thu => "Čtvrtek",
        Weekday::Fri => "Pátek",
        Weekday::Sat => "Sobota",
        Weekday::Sun => "Neděle",
    }
}

pub fn save_article_file_name(title: &String) -> String {
    title
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

pub fn formatted_article_date(now: DateTime<Local>) -> String {
    let day_name = day_of_week(now);
    let month_name_genitive = get_czech_month_genitive(now.month());

    format!(
        "{} {}. {} {}",
        day_name,
        now.day(),
        month_name_genitive,
        now.year()
    )
}

pub fn read_related_articles(related_articles: &Vec<String>) -> String {
    let mut related_article_snippets = String::new();
    for path in related_articles {
        let snippet_path = format!("snippets/{}.txt", path);
        if let Ok(snippet_html) = fs::read_to_string(&snippet_path) {
            related_article_snippets.push_str(&snippet_html);
            related_article_snippets.push('\n');
        }
    }
    related_article_snippets
}
