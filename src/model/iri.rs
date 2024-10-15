use crate::utils::twine::Twine;

#[derive(Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct Iri {
    iri: Twine
}