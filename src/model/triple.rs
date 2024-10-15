use crate::model::iri::Iri;
use crate::model::node::{Entity, Node};

#[derive(Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct Triple {
    pub(crate) subject: Entity,
    pub(crate) predicate: Iri,
    pub(crate) object: Node
}