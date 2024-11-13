use std::io::Read;
use xml::name::OwnedName;
use xml::namespace::Namespace;
use xml::reader::XmlEvent;
use crate::error::PenyuError;
use crate::model::graph::MemoryGraph;

enum State {
    PreStart,
    Started,
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
                    State::Started => {
                        Err(PenyuError::new("Unexpected start document".to_string(), None))?;
                    }
                }
            }
            XmlEvent::ProcessingInstruction { .. } => { todo!() }
            XmlEvent::StartElement {
                name, attributes, namespace
            } => {
                println!("Start element: {:?}", name);
                todo!()
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

fn tag_is_rdf_rdf(tag: &OwnedName, namespace: &Namespace) -> bool {
    let ns_is_rdf =
        if let Some(prefix) = &tag.prefix {
            prefix == "rdf"
        } else {
            false
        };
    if tag.local_name == "RDF" && ns_is_rdf {
        true
    } else {
        todo!()
    }
}
#[cfg(test)]
mod tests {
    use std::env::home_dir;
    use std::path::PathBuf;
    use crate::model::graph::{Graph, MemoryGraph};

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