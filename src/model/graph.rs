use crate::model::triple::Triple;
use std::collections::BTreeSet;

pub trait Graph {
    fn iter(&self) -> impl Iterator<Item=&Triple>;
}

pub struct MemoryGraph {
    triples: BTreeSet<Triple>,
}

impl MemoryGraph {
    pub fn new() -> MemoryGraph {
        MemoryGraph {
            triples: BTreeSet::new()
        }
    }

    pub fn add(&mut self, triple: Triple) {
        self.triples.insert(triple);
    }
}

impl Graph for MemoryGraph {
    fn iter(&self) -> impl Iterator<Item=&Triple> {
        self.triples.iter()
    }
}