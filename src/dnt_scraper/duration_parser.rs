use chrono::Duration;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, i64 as nom_i64};
use nom::error::Error;
use nom::sequence::{delimited, separated_pair, terminated};
use std::ops::Add;

pub fn parse_duration(value: &str) -> Option<Duration> {
    if let Some(days) = parse_days(value) {
        Some(days)
    } else if let Some(hours) = parse_hours(value) {
        Some(hours)
    } else {
        parse_hours_and_minutes(value)
    }
}

fn parse_days(value: &str) -> Option<Duration> {
    let raw_parser = terminated(
        nom_i64,
        alt((
            tag::<_, _, Error<_>>(" dager"),
            tag::<_, _, Error<_>>(" dag"),
        )),
    );
    let mut parser = delimited(tag("("), raw_parser, tag(")"));
    parser(value)
        .ok()
        .map(|(_, days): (_, i64)| Duration::days(days))
}

fn parse_hours(value: &str) -> Option<Duration> {
    let raw_parser = terminated(
        nom_i64,
        alt((
            tag::<_, _, Error<_>>(" timer"),
            tag::<_, _, Error<_>>(" time"),
        )),
    );
    let mut parser = delimited(tag("("), raw_parser, tag(")"));
    parser(value)
        .ok()
        .map(|(_, hours): (_, i64)| Duration::hours(hours))
}

fn parse_hours_and_minutes(value: &str) -> Option<Duration> {
    let raw_parser = separated_pair(
        terminated(nom_i64, tag::<_, _, Error<_>>("t")),
        char(' '),
        terminated(nom_i64, tag::<_, _, Error<_>>("min")),
    );
    let mut parser = delimited(tag("("), raw_parser, tag(")"));
    parser(value)
        .ok()
        .map(|(_, (hours, minutes)): (_, (i64, i64))| {
            Duration::hours(hours).add(Duration::minutes(minutes))
        })
}

#[cfg(test)]
mod tests {
    use crate::dnt_scraper::duration_parser::{
        parse_days, parse_duration, parse_hours, parse_hours_and_minutes,
    };
    use chrono::Duration;

    #[test]
    fn days() {
        assert_eq!(parse_days("(1 dag)"), Some(Duration::days(1)));
        assert_eq!(parse_days("(2 dager)"), Some(Duration::days(2)));
    }

    #[test]
    fn hours() {
        assert_eq!(parse_hours("(1 time)"), Some(Duration::hours(1)));
        assert_eq!(parse_hours("(2 timer)"), Some(Duration::hours(2)));
    }

    #[test]
    fn hours_and_minutes() {
        assert_eq!(
            parse_hours_and_minutes("(1t 45min)"),
            Some(Duration::minutes(105))
        );
        assert_eq!(
            parse_hours_and_minutes("(2t 55min)"),
            Some(Duration::minutes(175))
        );
    }

    #[test]
    fn should_parse_duration() {
        assert_eq!(parse_duration("(1 dag)"), Some(Duration::days(1)));
        assert_eq!(parse_duration("(2 timer)"), Some(Duration::hours(2)));
        assert_eq!(parse_duration("(3t 15min)"), Some(Duration::minutes(195)));
    }
}
