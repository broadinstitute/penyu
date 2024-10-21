use crate::model::triple::Triple;
use std::collections::{BTreeMap, BTreeSet};
use crate::model::iri::Iri;

pub trait Graph {
    fn prefixes(&self) -> &BTreeMap<String, Iri>;
    fn triples(&self) -> impl Iterator<Item=&Triple>;
}

pub struct MemoryGraph {
    prefixes: BTreeMap<String, Iri>,
    triples: BTreeSet<Triple>,
}

impl MemoryGraph {
    pub fn new() -> MemoryGraph {
        MemoryGraph {
            prefixes: BTreeMap::new(),
            triples: BTreeSet::new()
        }
    }
    pub fn add_prefix(&mut self, prefix: String, iri: Iri) {
        self.prefixes.insert(prefix, iri);
    }

    pub fn add(&mut self, triple: Triple) {
        self.triples.insert(triple);
    }
}

impl Default for MemoryGraph {
    fn default() -> Self { MemoryGraph::new() }
}

impl Graph for MemoryGraph {
    fn prefixes(&self) -> &BTreeMap<String, Iri> { &self.prefixes }
    fn triples(&self) -> impl Iterator<Item=&Triple> {
        self.triples.iter()
    }
}