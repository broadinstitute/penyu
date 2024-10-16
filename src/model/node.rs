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

impl From<Entity> for Node {
    fn from(entity: Entity) -> Self {
        Node::Entity(entity)
    }
}

impl From<Literal> for Node {
    fn from(literal: Literal) -> Self {
        Node::Literal(literal)
    }
}

impl From<Iri> for Entity {
    fn from(iri: Iri) -> Self {
        Entity::Iri(iri)
    }
}

impl From<BlankNode> for Entity {
    fn from(blank_node: BlankNode) -> Self {
        Entity::BlankNode(blank_node)
    }
}

impl From<&str> for BlankNode {
    fn from(id: &str) -> Self {
        BlankNode { id: Arc::new(id.to_string()) }
    }
}
impl From<String> for BlankNode {
    fn from(id: String) -> Self {
        BlankNode { id: Arc::new(id) }
    }
}

