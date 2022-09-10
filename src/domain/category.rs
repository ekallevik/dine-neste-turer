use rusqlite::types::ToSqlOutput;
use rusqlite::ToSql;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

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
