use scraper::{ElementRef, Selector};
use std::str::FromStr;
use std::hash::Hash;
use std::collections::HashSet;

pub fn parse_html_to_set<T: FromStr + Eq + Hash>(html: ElementRef, selector: &Selector) -> HashSet<T> {
    match parse_html(html, selector) {
        None => HashSet::new(),
        Some(value) => value
            .split_whitespace()
            .filter_map(|token| T::from_str(token).ok())
            .collect()
    }
}

pub fn parse_html_to_t<T: FromStr>(html: ElementRef, selector: &Selector) -> Option<T> {
    match parse_html(html, selector) {
        None => None,
        Some(value) => T::from_str(&*value).ok()
    }
}

pub fn parse_html(html: ElementRef, selector: &Selector) -> Option<String> {
    html
        .select(selector)
        .map(|element| element.inner_html())
        .next()
        .map(|s| { s.trim().to_string() })
}

pub fn parse_html_to_string(html: ElementRef, selector: &Selector, default: &str) -> String {
    parse_html(html, selector).unwrap_or(default.to_string())
}
