use std::sync::Arc;
use crate::model::literal::Literal;
use crate::model::iri::Iri;

#[derive(Clone, Ord, PartialOrd, PartialEq, Eq)]
pub enum Node {
    Entity(Entity),
    Literal(Literal)
}

#[derive(Clone, Ord, PartialOrd, PartialEq, Eq)]
pub enum Entity {
    Iri(Iri),
    BlankNode(BlankNode),
}

#[derive(Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct BlankNode {
    id: Arc<String>
}

impl BlankNode {
    pub fn id(&self) -> &str { self.id.as_str() }
}

