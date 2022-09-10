use std::fmt::{Display, Formatter};
use std::str::FromStr;

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
