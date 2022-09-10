use std::fmt::{Display, Formatter};
use chrono::{Duration, NaiveDate};
use std::collections::HashSet;
use itertools::join;
use crate::domain::audience::Audience;
use crate::domain::category::Category;

#[derive(Debug)]
#[allow(unused)]
pub struct Activity {
    pub title: String,
    pub category: Option<Category>,
    pub date: Option<NaiveDate>,
    pub duration: Option<Duration>,
    pub description: Option<String>,
    pub audiences: HashSet<Audience>,
    pub organizer: String,
    pub source: String,
}

impl Activity {
    pub fn get_audiences_as_string(&self) -> String {
        let vec = Vec::from_iter(self.audiences.iter());
        join(vec, "; ")
    }
}

impl Display for Activity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}: {}", self.date.unwrap_or_default(), self.title, self.source)
    }
}

