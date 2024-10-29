use crate::model::iri::Iri;

pub const PREFIX: &str = "umls";
pub const NAMESPACE: &Iri = &Iri::new_str("http://id.nlm.nih.gov/mesh/");

pub enum Level {
    Concept, Lexical, String, Atom
}

impl Level {
    pub fn letter(&self) -> char {
        match self {
            Level::Concept => 'C',
            Level::Lexical => 'L',
            Level::String => 'S',
            Level::Atom => 'A',
        }
    }
    pub fn create_iri(&self, id: u32) -> Iri {
        NAMESPACE.join(format!("{}{:07}", self.letter(), id))
    }
}
