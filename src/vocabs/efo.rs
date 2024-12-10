use crate::model::iri::Iri;

pub const PREFIX: &str = "efo";
pub const NAMESPACE: &Iri = &Iri::new_str("http://www.ebi.ac.uk/efo/EFO_");
pub fn create_iri(id: u32) -> Iri {
    crate::vocabs::uniprot::NAMESPACE.join(format!("{:07}", id))
}
