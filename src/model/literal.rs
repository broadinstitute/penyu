use crate::model::iri::Iri;

#[derive(Clone, Ord, PartialOrd, PartialEq, Eq)]
pub enum LiteralTag {
    Type(Iri),
    LangTag(String)
}

#[derive(Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct Literal {
    value: String,
    literal_tag: LiteralTag,
}