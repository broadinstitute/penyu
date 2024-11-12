use crate::model::iri::Iri;
use crate::model::node::{Entity, Node};
use crate::model::triple::Triple;
use std::collections::BTreeMap;

pub trait Graph {
    fn prefixes(&self) -> &BTreeMap<String, Iri>;
    fn triples(&self) -> impl Iterator<Item=Triple>;
}

pub struct MemoryGraph {
    prefixes: BTreeMap<String, Iri>,
    triples: BTreeMap<Entity, BTreeMap<Iri, Node>>,
}

impl MemoryGraph {
    pub fn new() -> MemoryGraph {
        MemoryGraph {
            prefixes: BTreeMap::new(),
            triples: BTreeMap::new(),
        }
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
    fn prefixes(&self) -> &BTreeMap<String, Iri> { &self.prefixes }
    fn triples(&self) -> impl Iterator<Item=Triple> {
        self.triples.iter().flat_map(|(subject, predicates)| {
            predicates.iter().map(move |(predicate, object)| {
                Triple::create(subject.clone(), predicate.clone(), object.clone())
            })
        })
    }
}