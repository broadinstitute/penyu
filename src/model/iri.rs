use std::fmt::Display;
use strey::Strey;

#[derive(Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct Iri {
    pub(crate) iri: Strey,
}

impl Display for Iri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.iri)
    }
}

impl From<String> for Iri {
    fn from(string: String) -> Self {
        Iri { iri: Strey::from(string) }
    }
}

impl From<&'static str> for Iri {
    fn from(string: &'static str) -> Self {
        Iri { iri: Strey::from(string) }
    }
}