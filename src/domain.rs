use std::fmt::{Display, Formatter};
use chrono::NaiveDate;

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct Activity {
    pub title: String,
    pub category: Option<String>,
    pub date: Option<NaiveDate>,
    pub duration: Option<String>,
    pub description: Option<String>,
    pub audiences: Option<String>,
    pub organizer: Option<String>,
    pub source: String,
}

impl Display for Activity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}: {}", self.date.unwrap_or_default(), self.title, self.source)
    }
}
