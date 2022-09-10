use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Add;
use std::str::FromStr;
use scraper::{ElementRef, Selector};
use chrono::{Duration, NaiveDate};
use nom::branch::alt;
use nom::bytes::complete::{tag};
use nom::character::complete::char;
use nom::character::complete::i64 as nom_i64;
use nom::sequence::{delimited, separated_pair, terminated};
use paris::info;
use crate::domain::{Activity, Audience, Category};
use nom::error::Error;


pub fn scrap_activities(source: &str) -> Vec<Activity> {
    let response = reqwest::blocking::get(source)
        .unwrap()
        .text()
        .unwrap();

    let document = scraper::Html::parse_document(&response);
    let activities_selector = Selector::parse("a.aktivitet-item").unwrap();

    let activities: Vec<_> = document
        .select(&activities_selector)
        .map(parse_activity)
        .collect();

    info!("Found {} activities", activities.len());
    activities
}

fn parse_activity(activity_html: ElementRef) -> Activity {
    let title_selector = Selector::parse("div.info>h3").unwrap();
    let category_selector = Selector::parse("div.info>div.meta>div.category").unwrap();
    let date_selector = Selector::parse("div.info>div.meta>div.date>span.short").unwrap();
    let duration_selector = Selector::parse("div.info>div.meta>div.duration").unwrap();
    let description_selector = Selector::parse("div.info>div.description").unwrap();
    let audiences_selector = Selector::parse("div.info>div.meta-secondary>div.audiences>span>span.audiences").unwrap();
    let organizer_selector = Selector::parse("div.info>div.meta-secondary>div.organizer>span>span.organizer").unwrap();

    let title = activity_html
        .select(&title_selector)
        .map(|y| y.inner_html())
        .next()
        .unwrap();

    let duration = parse_html(activity_html, &duration_selector)
        .map(|value| parse_duration(&value));

    let duration = match duration {
        Some(v) => v,
        None => None
    };

    let description = parse_html(activity_html, &description_selector);

    let category = parse_html_to_t::<Category>(activity_html, &category_selector);
    let audiences = parse_html_to_set::<Audience>(activity_html, &audiences_selector);

    let organizer = parse_html_to_string(activity_html, &organizer_selector, "Ukjent arrangør");

    let date = parse_html(activity_html, &date_selector)
        .map(|date| NaiveDate::parse_from_str(date.as_str(), "%d.%m.%y").unwrap_or_default());

    Activity {
        title: String::from(title.trim()),
        source: String::from(activity_html.value().attr("href").unwrap_or_default()),
        category,
        date,
        duration,
        description,
        audiences,
        organizer,
    }
}

fn parse_html_to_set<T: FromStr + Eq + Hash>(html: ElementRef, selector: &Selector) -> HashSet<T> {
    match parse_html(html, selector) {
        None => HashSet::new(),
        Some(value) => value
            .split_whitespace()
            .filter_map(|token| T::from_str(token).ok())
            .collect()
    }
}

fn parse_html_to_t<T: FromStr>(html: ElementRef, selector: &Selector) -> Option<T> {
    match parse_html(html, selector) {
        None => None,
        Some(value) => T::from_str(&*value).ok()
    }
}

fn parse_html(html: ElementRef, selector: &Selector) -> Option<String> {
    html
        .select(selector)
        .map(|element| element.inner_html())
        .next()
        .map(|s| { s.trim().to_string() })
}

fn parse_html_to_string(html: ElementRef, selector: &Selector, default: &str) -> String {
    parse_html(html, selector).unwrap_or(default.to_string())
}

fn parse_duration(value: &str) -> Option<Duration> {
    if let Some(days) = parse_days(value) {
        Some(days)
    } else {
        if let Some(hours) = parse_hours(value) {
            Some(hours)
        } else {
            if let Some(minutes) = parse_hours_and_minutes(value) {
                Some(minutes)
            } else {
                None
            }
        }
    }
}


fn parse_days(value: &str) -> Option<Duration> {
    let raw_parser = terminated(nom_i64, alt((tag::<_, _, Error<_>>(" dager"), tag::<_, _, Error<_>>(" dag"))));
    let mut parser = delimited(tag("("), raw_parser, tag(")"));
    parser(value)
        .ok()
        .map(|(_, days): (_, i64)| Duration::days(days))
}

fn parse_hours(value: &str) -> Option<Duration> {
    let raw_parser = terminated(nom_i64, alt((tag::<_, _, Error<_>>(" timer"), tag::<_, _, Error<_>>(" time"))));
    let mut parser = delimited(tag("("), raw_parser, tag(")"));
    parser(value)
        .ok()
        .map(|(_, hours): (_, i64)| Duration::hours(hours))
}

fn parse_hours_and_minutes(value: &str) -> Option<Duration> {
    let raw_parser = separated_pair(terminated(nom_i64, tag::<_, _, Error<_>>("t")), char(' '), terminated(nom_i64, tag::<_, _, Error<_>>("min")));
    let mut parser = delimited(tag("("), raw_parser, tag(")"));
    parser(value)
        .ok()
        .map(|(_, (hours, minutes)): (_, (i64, i64))| Duration::hours(hours).add(Duration::minutes(minutes)))
}


#[cfg(test)]
mod tests {
    use chrono::Duration;
    use crate::dnt_scraper::{parse_days, parse_duration, parse_hours, parse_hours_and_minutes};

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
        assert_eq!(parse_hours_and_minutes("(1t 45min)"), Some(Duration::minutes(105)));
        assert_eq!(parse_hours_and_minutes("(2t 55min)"), Some(Duration::minutes(175)));
    }

    #[test]
    fn should_parse_duration() {
        assert_eq!(parse_duration("(1 dag)"), Some(Duration::days(1)));
        assert_eq!(parse_duration("(2 timer)"), Some(Duration::hours(2)));
        assert_eq!(parse_duration("(3t 15min)"), Some(Duration::minutes(195)));
    }
}
