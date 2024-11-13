use crate::model::iri::Iri;
use crate::model::node::{Entity, Node};
use crate::model::triple::Triple;
use std::collections::BTreeMap;

pub trait Graph {
    fn default_ns(&self) -> &Option<Iri>;
    fn prefixes(&self) -> &BTreeMap<String, Iri>;
    fn triples(&self) -> impl Iterator<Item=Triple>;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
}

pub struct MemoryGraph {
    default_ns: Option<Iri>,
    prefixes: BTreeMap<String, Iri>,
    triples: BTreeMap<Entity, BTreeMap<Iri, Node>>,
}

impl MemoryGraph {
    pub fn new() -> MemoryGraph {
        MemoryGraph {
            default_ns: None,
            prefixes: BTreeMap::new(),
            triples: BTreeMap::new(),
        }
    }
    pub fn set_default_ns(&mut self, default_ns: Iri) {
        self.default_ns = Some(default_ns);
    }
    pub fn add_prefix(&mut self, prefix: String, iri: Iri) {
        self.prefixes.insert(prefix, iri);
    }

    pub fn add_triple(&mut self, triple: Triple) {
        self.add(triple.subject, triple.predicate, triple.object);
    }
    pub fn add<S, P, O>(&mut self, subject: S, predicate: P, object: O)
    where
        S: Into<Entity>,
        P: Into<Iri>,
        O: Into<Node>,
    {
        let subject = subject.into();
        let predicate = predicate.into();
        let object = object.into();
        self.triples.entry(subject).or_default().insert(predicate, object);
    }
}

impl Default for MemoryGraph {
    fn default() -> Self { MemoryGraph::new() }
}

impl Graph for MemoryGraph {
    fn default_ns(&self) -> &Option<Iri> { &self.default_ns }
    fn prefixes(&self) -> &BTreeMap<String, Iri> { &self.prefixes }
    fn triples(&self) -> impl Iterator<Item=Triple> {
        self.triples.iter().flat_map(|(subject, predicates)| {
            predicates.iter().map(move |(predicate, object)| {
                Triple::create(subject.clone(), predicate.clone(), object.clone())
            })
        })
    }

    fn is_empty(&self) -> bool {
        self.triples.values().all(|predicates| predicates.is_empty())
    }

    fn len(&self) -> usize {
        self.triples.values().map(|predicates| predicates.len()).sum()
    }
}