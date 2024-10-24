use crate::model::iri::Iri;

pub const PREFIX: &str = "uniprot";
pub const NAMESPACE: &Iri = &Iri::new_str("http://purl.uniprot.org/core/");
pub fn create_iri(id: &str) -> Iri {
    NAMESPACE.join(id.to_string())
}
