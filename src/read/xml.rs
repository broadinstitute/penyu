use crate::error::PenyuError;
use crate::model::graph::{Graph, MemoryGraph};
use crate::model::iri::Iri;
use crate::model::literal::{Literal, LiteralTag};
use crate::model::node::{BlankNode, Entity, Node};
use crate::vocabs;
use crate::vocabs::rdf;
use std::io::Read;
use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;
use xml::namespace::Namespace;
use xml::reader::XmlEvent;

enum State {
    PreStart,
    Started,
    Rdf { stack: Stack, n_b_nodes: usize },
    PostRdf,
    PostEnd,
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
                            Err(PenyuError::from("Unexpected document start"))?
                        }
                    }
                }
                XmlEvent::ProcessingInstruction { .. } => { state }
                XmlEvent::StartElement {
                    name, attributes, namespace
                } => {
                    match state {
                        State::Started => {
                            parse_rdf_start(&mut graph, &name, &attributes, &namespace)?
                        }
                        State::Rdf { stack, mut n_b_nodes } => {
                            let stack =
                                parse_rdf(stack, name, &attributes, &mut graph, &mut n_b_nodes)?;
                            State::Rdf { stack, n_b_nodes }
                        }
                        _ => {
                            Err(PenyuError::from(
                                format!("Unexpected start tag {:?}", name)
                            ))?
                        }
                    }
                }
                XmlEvent::EndElement { name } => {
                    match state {
                        State::Rdf { stack, n_b_nodes } => {
                            if stack.is_empty() {
                                if tag_is_rdf_rdf(&name) {
                                    State::PostRdf
                                } else {
                                    err_unexpected_end_tag(&name)?
                                }
                            } else {
                                let stack = stack.pop()?;
                                State::Rdf { stack, n_b_nodes }
                            }
                        }
                        _ => {
                            err_unexpected_end_tag(&name)?
                        }
                    }
                }
                XmlEvent::CData(string) => {
                    handle_characters(&mut graph, state, string)?
                }
                XmlEvent::Comment(_) => { state }
                XmlEvent::Characters(string) => {
                    handle_characters(&mut graph, state, string)?
                }
                XmlEvent::Whitespace(_) => { state }
                XmlEvent::EndDocument => {
                    if let State::PostRdf = state {
                        State::PostEnd
                    } else {
                        Err(PenyuError::from("Unexpected end of document"))?
                    }
                }
            }
    }
    Ok(graph)
}

fn handle_characters(graph: &mut MemoryGraph, state: State, string: String)
    -> Result<State, PenyuError> {
    Ok(match state {
        State::Rdf { stack: Stack::P(Some(stack_p)), n_b_nodes } => {
            let StackP {
                stack_s, predicate, literal_tag
            } = *stack_p;
            let literal_tag =
                literal_tag.unwrap_or_else(||
                    LiteralTag::Type(vocabs::xsd::STRING.clone())
                );
            let literal = Literal::new(string, literal_tag);
            graph.add(&stack_s.subject, &predicate, Node::from(literal));
            let literal_tag: Option<LiteralTag> = None;
            let stack_p = StackP::new(stack_s, predicate, literal_tag);
            State::Rdf { stack: Stack::P(Some(Box::new(stack_p))), n_b_nodes }
        }
        _ => {
            Err(PenyuError::from(
                format!("Unexpected characters {}", string)
            ))?
        }
    })
}

fn err_unexpected_end_tag(name: &OwnedName) -> Result<State, PenyuError> {
    Err(PenyuError::from(format!("Unexpected end tag {:?}", name)))
}

fn parse_rdf(stack: Stack, name: OwnedName, attributes: &[OwnedAttribute],
             graph: &mut MemoryGraph, n_b_nodes: &mut usize) -> Result<Stack, PenyuError> {
    match stack {
        Stack::S(stack_s) => {
            let predicate = iri_from_tag(&name, graph)?;
            let object =
                iri_from_attribute(attributes, "resource", rdf::NAMESPACE, graph);
            let literal_tag: Option<LiteralTag> =
                if let Some(iri) = object {
                    graph.add(&stack_s.subject, &predicate, Node::from(Entity::from(iri)));
                    None
                } else {
                    let lang_tag =
                        string_from_attribute(attributes, "lang", rdf::NAMESPACE);
                    let datatype =
                        iri_from_attribute(attributes, "datatype", rdf::NAMESPACE,
                                           graph);
                    match (lang_tag, datatype) {
                        (Some(lang), None) => { Some(LiteralTag::LangTag(lang)) }
                        (None, Some(datatype)) => { Some(LiteralTag::Type(datatype)) }
                        (None, None) => { None }
                        (Some(_), Some(_)) => {
                            Err(PenyuError::from(
                                format!("Tag {:?} has both lang and datatype attributes",
                                        name)
                            ))?
                        }
                    }
                };
            Ok(Stack::P(Some(Box::new(StackP::new(stack_s, predicate, literal_tag)))))
        }
        Stack::P(stack_p) => {
            let class = iri_from_tag(&name, graph)?;
            let id =
                iri_from_attribute(attributes, "about", rdf::NAMESPACE, graph);
            let entity =
                match id {
                    Some(iri) => { Entity::from(iri) }
                    None => {
                        let b_node_id = format!("node{}", n_b_nodes);
                        *n_b_nodes += 1;
                        Entity::BlankNode(BlankNode::from(b_node_id))
                    }
                };
            graph.add(&entity, rdf::TYPE, class);
            if let Some(stack_p) = &stack_p {
                let object = &stack_p.stack_s.subject;
                let predicate = &stack_p.predicate;
                graph.add(&entity, predicate, object);
            }
            Ok(Stack::S(StackS::new(stack_p, entity)))
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

fn iri_from_attribute(attributes: &[OwnedAttribute], attribute_name: &str, attribute_ns: &Iri,
                      graph: &MemoryGraph)
                      -> Option<Iri> {
    let iri: Option<Iri> =
        string_from_attribute(attributes, attribute_name, attribute_ns)
            .map(Iri::from)
            .map(|mut iri| {
                for prefix in graph.prefixes().values() {
                    iri = prefix.maybe_use_as_prefix_for(iri);
                }
                iri
            });
    iri
}

fn string_from_attribute(attributes: &[OwnedAttribute], attribute_name: &str, attribute_ns: &Iri)
                         -> Option<String> {
    let mut value: Option<String> = None;
    for attribute in attributes {
        if attribute.name.local_name == attribute_name
            && has_ns(&attribute.name, attribute_ns) {
            value = Some(attribute.value.clone());
        }
    }
    value
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
        Ok(State::Rdf { stack: Stack::new(), n_b_nodes: 0 })
    } else {
        Err(PenyuError::new("Unexpected start element".to_string(), None))?
    }
}

struct StackS {
    stack_p: Option<Box<StackP>>,
    subject: Entity,
}

struct StackP {
    stack_s: StackS,
    predicate: Iri,
    literal_tag: Option<LiteralTag>,
}

enum Stack {
    S(StackS),
    P(Option<Box<StackP>>),
}

impl StackS {
    fn new(stack_p: Option<Box<StackP>>, subject: Entity) -> StackS { StackS { stack_p, subject } }
}

impl StackP {
    fn new(stack_s: StackS, predicate: Iri, literal_tag: Option<LiteralTag>) -> StackP {
        StackP { stack_s, predicate, literal_tag }
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
            Err(PenyuError::from(format!("Unexpected attribute {:?}", attribute)))?
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
        assert_eq!(graph.prefixes().len(), 32);
        assert_eq!(graph.len(), 942159);
    }
    #[test]
    fn read_efo() {
        let graph = read_ontology("efo.owl");
        assert_eq!(graph.prefixes().len(), 27);
        assert_eq!(graph.len(), 2076147);
    }
    #[test]
    fn read_clo() {
        let graph = read_ontology("clo.owl");
        assert_eq!(graph.prefixes().len(), 28);
        assert_eq!(graph.len(), 459084);
    }
    #[test]
    fn read_mondo() {
        let graph = read_ontology("mondo.owl");
        assert_eq!(graph.prefixes().len(), 18);
        assert_eq!(graph.len(), 2305807);
    }
}