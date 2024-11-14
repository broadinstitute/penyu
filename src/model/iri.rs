use std::fmt::Display;
use strey::Strey;

#[derive(Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct Iri {
    pub(crate) iri: Strey,
}

impl Iri {
    pub const fn new(iri: Strey) -> Iri {
        Iri { iri }
    }
    pub const fn new_str(string: &'static str) -> Iri {
        Iri { iri: Strey::new_str(string) }
    }
    pub const fn join_str(&'static self, local: &'static str) -> Iri {
        Iri::new(self.iri.join_str(local))
    }
    pub fn join(&'static self, local: String) -> Iri {
        Iri::new(self.iri.join(local))
    }
    pub fn same_as<B: AsRef<[u8]>>(&self, bytes: B) -> bool {
        self.iri.bytes().eq(bytes.as_ref().iter().copied())
    }
}

impl Display for Iri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.iri)
    }
}

impl From<&Iri> for Iri {
    fn from(iri: &Iri) -> Self {
        iri.clone()
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