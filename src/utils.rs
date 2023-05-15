use chrono::{NaiveDate, NaiveTime};
use regex::Regex;

use crate::native_date_time_range::NaiveDateTimeRange;

/// Parse a string and return a [`NaiveDateTimeRange`] containing both start and end.
pub fn parse_from_str(date_string: &str) -> Result<NaiveDateTimeRange, anyhow::Error> {
    let re = Regex::new(r".*(?P<date_num>\d{1,2})\s+(?P<month_fr>\S+)\s+(?P<year>\d{4})\D+(?P<from_time>\d{2}:\d{2}:\d{2})\s+à\s+(?P<to_time>\d{2}:\d{2}:\d{2})$").unwrap();
    let cap = match re.captures(&date_string) {
        Some(data) => data,
        None => panic!("Date format is invalid"),
    };
    let year: i32 = cap
        .name("year")
        .expect("Could get year")
        .as_str()
        .parse()
        .expect("Could not parse the year as i32");
    let month_int: u32 = match cap
        .name("month_fr")
        .expect("Couldn't get month")
        .as_str()
        .to_lowercase()
        .as_str()
    {
        "janvier" => 1,
        "février"|"fevrier" => 2,
        "mars" => 3,
        "avril" => 4,
        "mai" => 5,
        "juin" => 6,
        "juillet" => 7,
        "août"|"aout" => 8,
        "septembre" => 9,
        "octobre" => 10,
        "novembre" => 11,
        "décembre"|"decembre" => 12,
        _ => panic!("Could not parse the month from str"),
    };
    let date_num: u32 = cap
        .name("date_num")
        .expect("Couldn't get date number")
        .as_str()
        .parse()
        .unwrap();

    let from_time =
        NaiveTime::parse_from_str(cap.name("from_time").unwrap().as_str(), "%H:%M:%S").unwrap();
    let to_time =
        NaiveTime::parse_from_str(cap.name("to_time").unwrap().as_str(), "%H:%M:%S").unwrap();

    let date =
        NaiveDate::from_ymd_opt(year, month_int, date_num).expect("Could not build NaiveDate");
    let start = date.and_time(from_time);
    let end = date.and_time(to_time);
    Ok(NaiveDateTimeRange::new_from_range(start, end))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_french_localized_date_range() {
        let t = parse_from_str("Le Dimanche 7 août 2022 de  09:00:00 à  15:00:00").unwrap();
        assert_eq!(t.get_start().to_string(), "2022-08-07 09:00:00");
        assert_eq!(t.get_end().to_string(), "2022-08-07 15:00:00");
    }
}
