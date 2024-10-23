use crate::model::iri::Iri;

pub const PREFIX: &str = "obo";
pub const NAMESPACE: &Iri = &Iri::new_str("http://purl.obolibrary.org/obo/");

pub mod ns {
    use crate::model::iri::Iri;
    use crate::vocabs::obo::NAMESPACE;

    pub const RO: &Iri = &NAMESPACE.join_str("RO_");
    pub const UBERON: &Iri = &NAMESPACE.join_str("UBERON_");

}

pub enum Ontology {
    RO, UBERON
}

impl Ontology {
    pub fn namespace(&self) -> &'static Iri {
        match self {
            Ontology::RO => ns::RO,
            Ontology::UBERON => ns::UBERON
        }
    }
    pub fn create_iri(&self, id: u64) -> Iri {
        self.namespace().join(format!("{:07}", id))
    }
}

