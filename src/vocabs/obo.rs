use crate::model::iri::Iri;

pub const PREFIX: &str = "obo";
pub const NAMESPACE: &Iri = &Iri::new_str("http://purl.obolibrary.org/obo/");

pub fn create_iri(id: u64) -> Iri {
    NAMESPACE.join(format!("RO_{:07}", id))
}

