use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use chrono::{Duration, NaiveDate};
use itertools::join;
use rusqlite::ToSql;
use rusqlite::types::ToSqlOutput;

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

impl Display for Activity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}: {}", self.date.unwrap_or_default(), self.title, self.source)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Audience {
    Barn,
    Ungdom,
    Voksne,
    Seniorer,
    Fjellsportinteresserte,
    Funksjonshemmede,
    Utviklingshemmede,
}

impl Activity {

    pub fn get_audiences_as_string(&self) -> String {
        let vec = Vec::from_iter(self.audiences.iter());
        join(vec, "; ")
    }

}

impl FromStr for Audience {
    type Err = ();

    fn from_str(input: &str) -> Result<Audience, Self::Err> {
        match input {
            "Barn" => Ok(Audience::Barn),
            "Ungdom" => Ok(Audience::Ungdom),
            "Voksne" => Ok(Audience::Voksne),
            "Fjellsportinteresserte" => Ok(Audience::Fjellsportinteresserte),
            "Seniorer" => Ok(Audience::Seniorer),
            "Funksjonshemmede" => Ok(Audience::Funksjonshemmede),
            "Utviklingshemmede" => Ok(Audience::Utviklingshemmede),
            _ => Err(()),
        }
    }
}

impl Display for Audience {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Audience::Barn => write!(f, "Barn"),
            Audience::Ungdom => write!(f, "Ungdom"),
            Audience::Voksne => write!(f, "Voksne"),
            Audience::Seniorer => write!(f, "Seniorer"),
            Audience::Fjellsportinteresserte => write!(f, "Fjellsportinteresserte"),
            Audience::Funksjonshemmede => write!(f, "Funksjonshemmede"),
            Audience::Utviklingshemmede => write!(f, "Utviklingshemmede"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Category {
    Annet,
    Arrangement,
    Dugnad,
    Fellstur,
    Kurs,
}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::Annet => write!(f, "Annet"),
            Category::Arrangement => write!(f, "Arrangement"),
            Category::Dugnad => write!(f, "Dugnad"),
            Category::Fellstur => write!(f, "Fellestur"),
            Category::Kurs => write!(f, "Kurs"),
        }
    }
}

impl ToSql for Category {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.to_string()))
    }
}

impl FromStr for Category {
    type Err = ();

    fn from_str(input: &str) -> Result<Category, Self::Err> {
        match input {
            "Annet" => Ok(Category::Annet),
            "Arrangement" => Ok(Category::Arrangement),
            "Dugnad" => Ok(Category::Dugnad),
            "Fellestur" => Ok(Category::Fellstur),
            "Kurs" => Ok(Category::Kurs),
            _ => Err(()),
        }
    }
}
