use std::fmt::Display;
use crate::utils::twine::Twine;

#[derive(Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct Iri {
    iri: Twine
}

impl Iri {
    pub(crate) fn as_str(&self) -> &str {
        self.iri.as_str()
    }
}

impl Display for Iri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.iri)
    }
}