use crate::dnt_scraper::{duration_parser, html_parser};
use crate::domain::activity::Activity;
use crate::domain::audience::Audience;
use crate::domain::category::Category;
use chrono::NaiveDate;
use paris::info;
use scraper::{ElementRef, Selector};

pub fn scrap_activities(source: &str) -> Vec<Activity> {
    let response = reqwest::blocking::get(source).unwrap().text().unwrap();

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
    let audiences_selector =
        Selector::parse("div.info>div.meta-secondary>div.audiences>span>span.audiences").unwrap();
    let organizer_selector =
        Selector::parse("div.info>div.meta-secondary>div.organizer>span>span.organizer").unwrap();

    let title = activity_html
        .select(&title_selector)
        .map(|y| y.inner_html())
        .next()
        .unwrap();

    let duration = html_parser::parse_html(activity_html, &duration_selector)
        .map(|value| duration_parser::parse_duration(&value));

    let duration = match duration {
        Some(v) => v,
        None => None,
    };

    let description = html_parser::parse_html(activity_html, &description_selector);

    let category = html_parser::parse_html_to_t::<Category>(activity_html, &category_selector);
    let audiences = html_parser::parse_html_to_set::<Audience>(activity_html, &audiences_selector);

    let organizer =
        html_parser::parse_html_to_string(activity_html, &organizer_selector, "Ukjent arrangør");

    let date = html_parser::parse_html(activity_html, &date_selector)
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
