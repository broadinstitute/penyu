use crate::model::iri::Iri;

pub const PREFIX: &str = "obo";
pub const NAMESPACE: &Iri = &Iri::new_str("http://purl.obolibrary.org/obo/");

pub mod ns {
    use crate::model::iri::Iri;
    use crate::vocabs::obo::NAMESPACE;

    pub const RO: &Iri = &NAMESPACE.join_str("RO_");
    pub const UBERON: &Iri = &NAMESPACE.join_str("UBERON_");
    pub const MONDO: &Iri = &NAMESPACE.join_str("MONDO_");
    pub const GENO: &Iri = &NAMESPACE.join_str("GENO_");

}

pub enum Ontology {
    RO, UBERON, MONDO, GENO
}

impl Ontology {
    pub fn namespace(&self) -> &'static Iri {
        match self {
            Ontology::RO => ns::RO,
            Ontology::UBERON => ns::UBERON,
            Ontology::MONDO => ns::MONDO,
            Ontology::GENO => ns::GENO,
        }
    }
    pub fn create_iri(&self, id: u64) -> Iri {
        self.namespace().join(format!("{:07}", id))
    }
}

