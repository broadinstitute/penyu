use crate::model::iri::Iri;
use crate::vocabs;

#[derive(Clone, Ord, PartialOrd, PartialEq, Eq)]
pub enum LiteralTag {
    Type(Iri),
    LangTag(String)
}

#[derive(Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct Literal {
    pub(crate) string: String,
    pub(crate) literal_tag: LiteralTag,
}

impl From<String> for Literal {
    fn from(string: String) -> Self {
        Literal { string, literal_tag: LiteralTag::Type(vocabs::xsd::STRING.clone()) }
    }
}

impl From<f64> for Literal {
    fn from(float: f64) -> Self {
        Literal {
            string: float.to_string(), literal_tag: LiteralTag::Type(vocabs::xsd::DOUBLE.clone())
        }
    }
}