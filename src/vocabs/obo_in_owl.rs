use crate::model::iri::Iri;

pub const PREFIX: &str = "oboInOwl";
pub const NAMESPACE: &Iri =
    &Iri::new_str("http://www.geneontology.org/formats/oboInOwl#");

pub const HAS_EXACT_SYNONYM: &Iri = &NAMESPACE.join_str("hasExactSynonym");
