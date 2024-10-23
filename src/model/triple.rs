use crate::model::iri::Iri;
use crate::model::node::{Entity, Node};

#[derive(Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct Triple {
    pub(crate) subject: Entity,
    pub(crate) predicate: Iri,
    pub(crate) object: Node,
}

impl Triple {
    pub fn new(subject: Entity, predicate: Iri, object: Node) -> Self {
        Triple { subject, predicate, object }
    }
    pub fn create<S, P, O>(subject: S, predicate: P, object: O) -> Self
    where
        S: Into<Entity>,
        P: Into<Iri>,
        O: Into<Node>,
    {
        Triple::new(subject.into(), predicate.into(),object.into())
    }
}