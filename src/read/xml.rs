use crate::error::PenyuError;
use crate::model::graph::MemoryGraph;
use crate::model::iri::Iri;
use crate::vocabs;
use std::io::Read;
use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;
use xml::reader::XmlEvent;
use crate::model::node::Node;

enum State {
    PreStart,
    Started,
    Rdf(Stack)
}


pub fn read<R: Read>(read: &mut R) -> Result<MemoryGraph, PenyuError> {
    let mut graph = MemoryGraph::new();
    let parser = xml::EventReader::new(read);
    let mut state = State::PreStart;
    for event in parser {
        let event = event?;
        match event {
            XmlEvent::StartDocument { .. } => {
                match state {
                    State::PreStart => { state = State::Started; }
                    _ => {
                        Err(PenyuError::new("Unexpected start document".to_string(), None))?;
                    }
                }
            }
            XmlEvent::ProcessingInstruction { .. } => { todo!() }
            XmlEvent::StartElement {
                name, attributes, namespace
            } => {
                match state {
                    State::Started => {
                        state = parse_rdf_start(&mut graph, &name, &attributes)?;
                    }
                    State::Rdf(ref mut stack) => {
                        parse_rdf(stack, name);
                    }
                    _ => {
                        Err(PenyuError::new("Unexpected start element".to_string(), None))?;
                    }
                }
            }
            XmlEvent::EndElement { .. } => { todo!() }
            XmlEvent::CData(_) => { todo!() }
            XmlEvent::Comment(_) => { /* Do nothing */ }
            XmlEvent::Characters(_) => { todo!() }
            XmlEvent::Whitespace(_) => { /* Do nothing */ }
            XmlEvent::EndDocument => { todo!() }
        }
    }
    Ok(graph)
}

fn parse_rdf(stack: &mut Stack, name: OwnedName) {
    match stack {
        Stack::S(_) => {
            println!("Element: {:?}", name);
            todo!("Parse next predicate")
        }
        Stack::P(_) => {
            println!("Element: {:?}", name);
            todo!("Parse next object")
        }
    }
}

fn parse_rdf_start(mut graph: &mut MemoryGraph, name: &OwnedName, attributes: &Vec<OwnedAttribute>)
                   -> Result<State, PenyuError> {
    if tag_is_rdf_rdf(&name) {
        parse_attributes_top_level(&mut graph, &attributes)?;
        Ok(State::Rdf(Stack::new()))
    } else {
        Err(PenyuError::new("Unexpected start element".to_string(), None))?
    }
}

struct StackS {
    stack_p: Option<Box<StackP>>,
    subject: Node
}

struct StackP {
    stack_s: StackS,
    predicate: Iri,
}

enum Stack {
    S(StackS),
    P(Option<Box<StackP>>),
}

impl StackS {
    fn new(subject: Node) -> StackS {
        StackS {
            stack_p: None,
            subject
        }
    }
}

impl StackP {
    fn new(stack_s: StackS, predicate: Iri) -> StackP {
        StackP {
            stack_s,
            predicate
        }
    }
}

impl Stack {
    fn new() -> Stack {
        Stack::P(None)
    }
    fn pop(self) -> Result<Self, PenyuError> {
        match self {
            Stack::S(stack_s) => {
                Ok(Stack::P(stack_s.stack_p))
            }
            Stack::P(Some(stack_p)) => {
                Ok(Stack::S(stack_p.stack_s))
            }
            Stack::P(None) => {
                Err(PenyuError::new("Stack is empty".to_string(), None))
            }
        }
    }
}

fn parse_attributes_top_level(graph: &mut MemoryGraph, attributes: &[OwnedAttribute])
    -> Result<(), PenyuError> {
    for attribute in attributes {
        if attribute.name.local_name == "base" && has_ns(&attribute.name, vocabs::xml::NAMESPACE) {
            let base = Iri::from(attribute.value.clone());
            graph.set_base_ns(base);
        } else {
            println!("Attribute: {:?}", attribute);
            todo!("Read attributes");
        }
    }
    Ok(())
}

fn tag_is_rdf_rdf(tag: &OwnedName) -> bool {
    tag.local_name == "RDF" && has_ns(tag, vocabs::rdf::NAMESPACE)
}

fn has_ns(tag: &OwnedName, ns: &Iri) -> bool {
    tag.namespace.as_ref().is_some_and(|s| ns.same_as(s))
}

#[cfg(test)]
mod tests {
    use crate::model::graph::{Graph, MemoryGraph};
    use std::env::home_dir;
    use std::path::PathBuf;

    fn ontologies_dir() -> PathBuf {
        home_dir().unwrap().join("lembic").join("ontos")
    }

    fn read_ontology(file: &str) -> MemoryGraph {
        let uberon = ontologies_dir().join(file);
        let file = std::fs::File::open(uberon).unwrap();
        super::read(&mut std::io::BufReader::new(file)).unwrap()
    }

    #[test]
    fn read_uberon() {
        let graph = read_ontology("uberon.owl");
        assert!(graph.prefixes().len() > 10);
        assert!(graph.len() > 100);
    }
    #[test]
    fn read_efo() {
        let graph = read_ontology("efo.owl");
        assert!(graph.prefixes().len() > 10);
        assert!(graph.len() > 100);
    }
    #[test]
    fn read_clo() {
        let graph = read_ontology("clo.owl");
        assert!(graph.prefixes().len() > 10);
        assert!(graph.len() > 100);
    }
    #[test]
    fn read_mondo() {
        let graph = read_ontology("mondo.owl");
        assert!(graph.prefixes().len() > 10);
        assert!(graph.len() > 100);
    }
}