use chrono::offset::Local;
use chrono::{DateTime, TimeZone};
use chrono::Datelike;
use chrono::Timelike;
use regex::Regex;
use std::ops::Add;

#[derive(Debug, Clone)]
enum TokenType {
    Number(u8),
    AmPm(bool), // false: am ; true: pm
    Month(u8),
    Weekday(u8),       // from sunday, one-indexed
    TimeOfDay(u8, u8), // hour, min
    RelativeDays(i8),  // days forward/backward
    Midnight,
    Unknown,
}

pub fn parse_time(time: &str) -> Result<DateTime<Local>, &str> {
    let match_time = Regex::new(r"^\d\d?:\d{2}$").unwrap();
    let match_number = Regex::new(r"^\d+$").unwrap();

    let tokens = time.split(" ").map(|e| e.to_lowercase());

    use TokenType::*;

    let parsed_vec: Vec<TokenType> = tokens
        .map(|token| match token.trim().as_ref() {
            "jan" | "january" => Month(1),
            "feb" | "febraury" => Month(2),
            "mar" | "march" => Month(3),
            "apr" | "april" => Month(4),
            "may" => Month(5),
            "jun" | "june" => Month(6),
            "jul" | "july" => Month(7),
            "aug" | "august" => Month(8),
            "sep" | "sept" | "september" => Month(9),
            "oct" | "october" => Month(10),
            "nov" | "november" => Month(11),
            "dec" | "december" => Month(12),

            "noon" => TimeOfDay(12, 0),
            "midnight" => Midnight,

            "mon" | "monday" => Weekday(1),
            "tue" | "tues" | "tuesday" => Weekday(2),
            "wed" | "wednesday" => Weekday(3),
            "thu" | "thur" | "thurs" | "thursday" => Weekday(4),
            "fri" | "friday" => Weekday(5),
            "sat" | "saturday" => Weekday(6),
            "sun" | "sunday" => Weekday(7),

            "tod" | "today" => RelativeDays(0),
            "tom" | "tomorrow" => RelativeDays(1),
            "yesterday" => RelativeDays(-1),

            "am" => AmPm(false),
            "pm" => AmPm(true),

            _ => {
                if match_time.is_match(token.as_ref()) {
                    let parts: Vec<&str> = token.split(":").collect();
                    let hour = parts[0].parse::<u8>().unwrap();
                    let min = parts[1].parse::<u8>().unwrap();
                    TimeOfDay(hour, min)
                } else if match_number.is_match(token.as_ref()) {
                    let val = token.parse::<u8>().unwrap();
                    Number(val)
                } else {
                    Unknown
                }
            }
        })
        .collect();

    let has_time = parsed_vec.clone().into_iter().any(|x| match x {
        AmPm(_) => true,
        TimeOfDay(_, _) => true,
        Midnight => true,
        _ => false,
    });

    let mut parsed = parsed_vec.into_iter().peekable();

    // dayOffset is for midnight
    // (hour, min, dayOffset)
    let time: (u8, u8, bool) = if has_time {
        match parsed.next() {
            Some(tok) => match tok {
                Number(hour) => match parsed.peek() {
                    Some(tok) => match tok {
                        AmPm(am_pm) => match am_pm {
                            false => {
                                parsed.next();
                                Ok((if hour == 12 { 0 } else { hour }, 0, false))
                            }
                            true => {
                                parsed.next();
                                Ok((if hour == 12 { 12 } else { hour + 12 }, 0, false))
                            }
                        },
                        _ => Err("Expected am/pm after number"),
                    },
                    None => Err("Unexpected EOF after number"),
                },
                TimeOfDay(hour, min) => match parsed.peek() {
                    Some(tok) => match tok {
                        AmPm(am_pm) => match am_pm {
                            false => {
                                parsed.next();
                                Ok((if hour == 12 { 0 } else { hour }, min, false))
                            }
                            true => {
                                parsed.next();
                                Ok((if hour == 12 { 12 } else { hour + 12 }, min, false))
                            }
                        },
                        _ => Ok((hour, min, false)),
                    },
                    None => Ok((hour, min, false)),
                },
                Midnight => Ok((0, 0, true)),
                _ => Ok((0, 0, true)),
            },
            None => Err("Unexpected EOF"),
        }?
    } else {
        (0, 0, true)
    };

    match time {
        (hour, min, day_offset) => {
            let today = chrono::offset::Local::today();
            let spec_date = match parsed.next() {
                Some(tok) => match tok {
                    RelativeDays(n) => Some(today.add(chrono::Duration::days(n as i64))),
                    Number(year_or_day) => match parsed.next() {
                        Some(tok) => match tok {
                            Month(month) => Some(Local.ymd(today.year() as i32, month as u32, year_or_day as u32)),
                            _ => None,
                        },
                        None => Some(Local.ymd(year_or_day as i32, 1, 1)),
                    },
                    _ => None,
                },
                None => None,
            };
            let date = match spec_date {
                Some(d) => d,
                None => today,
            };
            let datetime = date.and_hms(hour as u32, min as u32, 0);
            if day_offset {
                Ok(datetime.add(chrono::Duration::days(1)))
            } else {
                Ok(datetime)
            }
        }
    }
}

pub fn format_relative_time(to: DateTime<Local>) -> String {
    let now = chrono::offset::Local::now();
    let diff = to.signed_duration_since(now);

    if to.day() == now.day() {
        return to.format("%-I:%M %P").to_string();
    } else if to.day() == now.day() + 1 {
        return if to.hour() == 0 && to.minute() == 0 {
            "Today".to_string()
        } else {
            to.format("Tom (%b %-d) %-I:%M %P").to_string()
        } 
    }

    let days = diff.num_days();
    if days <= 7 {
        return if to.hour() == 0 && to.minute() == 0 {
            to.date().pred().format("%A").to_string()
        } else {
            to.format("%a %-I:%M %P").to_string()
        }
    }

    let weeks = diff.num_weeks();
    if weeks.abs() < 8 {
        return if to.hour() == 0 && to.minute() == 0 {
            to.date().pred().format("%b %-d").to_string()
        } else {
            to.format("%b %-d").to_string()
        }
    }

    return format!("{:?}", to);
}

pub fn format_time_distance(to: DateTime<Local>) -> String {
    let now = chrono::offset::Local::now();
    let diff = to.signed_duration_since(now);

    let weeks = diff.num_weeks();
    if weeks.abs() > 2 {
        return format!("{}w", weeks);
    }

    let days = diff.num_days();
    if days > 2 {
        return format!("{}d", days);
    }

    let minutes = diff.num_minutes();
    if minutes > 90 {
        let hours = diff.num_hours();
        return format!("{}h", hours);
    } else {
        return format!("{}m", minutes);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_today_time_am() {
        assert_eq!(
            parse_time("10 am"),
            Ok(chrono::offset::Local::today().and_hms(10, 0, 0))
        );
    }

    #[test]
    fn test_today_time_am_min() {
        assert_eq!(
            parse_time("2:30 am"),
            Ok(chrono::offset::Local::today().and_hms(2, 30, 0))
        );
    }

    #[test]
    fn test_today_time_12_am() {
        assert_eq!(
            parse_time("12:30 am"),
            Ok(chrono::offset::Local::today().and_hms(0, 30, 0))
        );
    }

    #[test]
    fn test_today_time_12_pm() {
        assert_eq!(
            parse_time("12:30 pm"),
            Ok(chrono::offset::Local::today().and_hms(12, 30, 0))
        );
    }

    #[test]
    fn test_today_time_24hr() {
        assert_eq!(
            parse_time("22:30"),
            Ok(chrono::offset::Local::today().and_hms(22, 30, 0))
        );
    }
}
