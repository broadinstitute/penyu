use std::borrow::Cow;
use std::sync::Arc;

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
pub struct Iri {
    iri: Arc<Cow<'static, String>>
}

#[derive(Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct BlankNode {
    id: Arc<String>
}

#[derive(Clone, Ord, PartialOrd, PartialEq, Eq)]
pub enum LiteralType {
    Iri(Iri),
    LangTag(String)
}
#[derive(Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct Literal {
    value: String,
    literal_type: LiteralType,
}

#[derive(Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct Triple {
    subject: Entity,
    predicate: Iri,
    object: Node
}

