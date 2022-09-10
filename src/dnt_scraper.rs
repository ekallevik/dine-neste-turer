use scraper::{ElementRef, Selector};
use chrono::NaiveDate;
use paris::info;
use crate::domain::Activity;

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

    let category = parse_html(activity_html, &category_selector);
    let duration = parse_html(activity_html, &duration_selector);
    let description = parse_html(activity_html, &description_selector);
    let audiences = parse_html(activity_html, &audiences_selector);
    let organizer = parse_html(activity_html, &organizer_selector);

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

fn parse_html(html: ElementRef, selector: &Selector) -> Option<String> {
    html
        .select(selector)
        .map(|element| element.inner_html())
        .next()
        .map(|s| { s.trim().to_string() })
}
