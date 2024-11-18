use crate::error::PenyuError;
use crate::model::graph::{Graph, MemoryGraph};
use crate::model::iri::Iri;
use crate::model::node::{Entity, Node};
use crate::vocabs;
use std::io::Read;
use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;
use xml::namespace::Namespace;
use xml::reader::XmlEvent;

enum State {
    PreStart,
    Started,
    Rdf(Stack),
    PostRdf,
    PostEnd
}


pub fn read<R: Read>(read: &mut R) -> Result<MemoryGraph, PenyuError> {
    let mut graph = MemoryGraph::new();
    let parser = xml::EventReader::new(read);
    let mut state = State::PreStart;
    for event in parser {
        let event = event?;
        state =
            match event {
                XmlEvent::StartDocument { .. } => {
                    match state {
                        State::PreStart => { State::Started }
                        _ => {
                            Err(PenyuError::new("Unexpected start document".to_string(), None))?
                        }
                    }
                }
                XmlEvent::ProcessingInstruction { .. } => { todo!() }
                XmlEvent::StartElement {
                    name, attributes, namespace
                } => {
                    match state {
                        State::Started => {
                            parse_rdf_start(&mut graph, &name, &attributes, &namespace)?
                        }
                        State::Rdf(stack) => {
                            let stack = parse_rdf(stack, name, &attributes, &mut graph)?;
                            State::Rdf(stack)
                        }
                        _ => {
                            Err(PenyuError::new("Unexpected start element".to_string(), None))?
                        }
                    }
                }
                XmlEvent::EndElement { name } => {
                    match state {
                        State::Rdf(stack) => {
                            if stack.is_empty() {
                                if tag_is_rdf_rdf(&name) {
                                    State::PostRdf
                                } else {
                                    Err(PenyuError::new("Unexpected end element".to_string(), None))?
                                }
                            } else {
                                let stack = stack.pop()?;
                                State::Rdf(stack)
                            }
                        }
                        _ => {
                            Err(PenyuError::new("Unexpected end element".to_string(), None))?
                        }
                    }
                }
                XmlEvent::CData(_) => { todo!() }
                XmlEvent::Comment(_) => { state }
                XmlEvent::Characters(_) => { todo!() }
                XmlEvent::Whitespace(_) => { state }
                XmlEvent::EndDocument => { todo!() }
            }
    }
    Ok(graph)
}

fn parse_rdf(stack: Stack, name: OwnedName, attributes: &[OwnedAttribute],
             graph: &mut MemoryGraph) -> Result<Stack, PenyuError> {
    match stack {
        Stack::S(stack_s) => {
            let predicate = iri_from_tag(&name, graph)?;
            Ok(Stack::P(Some(Box::new(StackP::new(stack_s, predicate)))))
        }
        Stack::P(stack_p) => {
            let class = iri_from_tag(&name, graph)?;
            let id = iri_from_about(&name, attributes, graph)?;
            graph.add(&id, vocabs::rdf::TYPE, class);
            if let Some(stack_p) = &stack_p {
                let object = &stack_p.stack_s.subject;
                let predicate = &stack_p.predicate;
                graph.add(&id, predicate, object);
            }
            Ok(Stack::S(StackS::new(stack_p, Node::from(Entity::from(id)))))
        }
    }
}

fn iri_from_tag(name: &OwnedName, graph: &mut MemoryGraph) -> Result<Iri, PenyuError> {
    match &name.namespace {
        None => {
            Err(PenyuError::from(format!("No namespace for tag: {:?}", name)))
        }
        Some(ns) => {
            let empty_prefix = "".to_string();
            let prefix = name.prefix.as_ref().unwrap_or(&empty_prefix);
            let iri_get = graph.prefixes().get(prefix);
            let ns_iri =
                match iri_get {
                    None => {
                        let ns_iri = Iri::from(ns.clone());
                        graph.add_prefix(prefix.clone(), ns_iri.clone());
                        ns_iri
                    }
                    Some(ns_iri) => {
                        if !ns_iri.same_as(ns) {
                            Err(PenyuError::from(
                                format!("Prefix {} used for {}, but already bound to {}",
                                        prefix, ns, ns_iri)
                            ))?;
                        }
                        ns_iri.clone()
                    }
                };
            Ok(ns_iri.append(name.local_name.clone()))
        }
    }
}

fn iri_from_about(tag: &OwnedName, attributes: &[OwnedAttribute], graph: &MemoryGraph)
                  -> Result<Iri, PenyuError> {
    let mut about: Option<String> = None;
    for attribute in attributes {
        if attribute.name.local_name == "about" && has_ns(&attribute.name, vocabs::rdf::NAMESPACE) {
            about = Some(attribute.value.clone());
        } else {
            Err(PenyuError::from(
                format!("Unexpected attribute: {:?} of tag {:?}", attribute, tag)
            ))?;
        }
    }
    match about {
        Some(about) => {
            let mut about = Iri::from(about);
            for prefix in graph.prefixes().values() {
                about = prefix.maybe_use_as_prefix_for(about);
            }
            Ok(about)
        }
        None => {
            Err(PenyuError::from(format!("Tag {:?} has no rdf:about attribute", tag)))
        }
    }
}

fn parse_rdf_start(graph: &mut MemoryGraph, name: &OwnedName, attributes: &[OwnedAttribute],
                   namespace: &Namespace)
                   -> Result<State, PenyuError> {
    if tag_is_rdf_rdf(name) {
        for mapping in namespace.0.iter() {
            let prefix = mapping.0.clone();
            let ns_iri = Iri::from(mapping.1.clone());
            graph.add_prefix(prefix, ns_iri);
        }
        parse_attributes_top_level(graph, attributes)?;
        Ok(State::Rdf(Stack::new()))
    } else {
        Err(PenyuError::new("Unexpected start element".to_string(), None))?
    }
}

struct StackS {
    stack_p: Option<Box<StackP>>,
    subject: Node,
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
    fn new(stack_p: Option<Box<StackP>>, subject: Node) -> StackS { StackS { stack_p, subject, } }
}

impl StackP {
    fn new(stack_s: StackS, predicate: Iri) -> StackP {
        StackP {
            stack_s,
            predicate,
        }
    }
}

impl Stack {
    fn new() -> Stack {
        Stack::P(None)
    }

    fn is_empty(&self) -> bool {
        match self {
            Stack::S(_) => { false }
            Stack::P(stack_p) => { stack_p.is_none() }
        }
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