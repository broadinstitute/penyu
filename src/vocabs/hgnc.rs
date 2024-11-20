use crate::model::iri::Iri;

pub const PREFIX: &str = "hgnc";
pub const NAMESPACE: &Iri = &Iri::new_str("http://identifiers.org/hgnc/");

pub fn create_iri(id: u32) -> Iri {
    NAMESPACE.join(id.to_string())
}