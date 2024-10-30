use crate::model::iri::Iri;

pub const PREFIX: &str = "obo";
pub const NAMESPACE: &Iri = &Iri::new_str("http://purl.obolibrary.org/obo/");

pub mod ns {
    use crate::model::iri::Iri;
    use crate::vocabs::obo::NAMESPACE;

    pub const GENO: &Iri = &NAMESPACE.join_str("GENO_");
    pub const MONDO: &Iri = &NAMESPACE.join_str("MONDO_");
    pub const RO: &Iri = &NAMESPACE.join_str("RO_");
    pub const SO: &Iri = &NAMESPACE.join_str("SO_");
    pub const UBERON: &Iri = &NAMESPACE.join_str("UBERON_");

}

pub enum Ontology {
    GENO, MONDO, RO, SO, UBERON
}

impl Ontology {
    pub fn namespace(&self) -> &'static Iri {
        match self {
            Ontology::GENO => ns::GENO,
            Ontology::MONDO => ns::MONDO,
            Ontology::RO => ns::RO,
            Ontology::SO => ns::SO,
            Ontology::UBERON => ns::UBERON,
        }
    }
    pub fn create_iri(&self, id: u32) -> Iri {
        self.namespace().join(format!("{:07}", id))
    }
}

