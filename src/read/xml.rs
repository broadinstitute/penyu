use std::io::Read;
use xml::reader::XmlEvent;
use crate::error::PenyuError;
use crate::model::graph::MemoryGraph;

pub fn read<R: Read>(read: &mut R) -> Result<MemoryGraph, PenyuError> {
    let mut graph = MemoryGraph::new();
    let parser = xml::EventReader::new(read);
    for event in parser {
        let event = event?;
        match event {
            XmlEvent::StartDocument { .. } => {}
            XmlEvent::ProcessingInstruction { .. } => {}
            XmlEvent::StartElement { .. } => {}
            XmlEvent::EndElement { .. } => {}
            XmlEvent::CData(_) => {}
            XmlEvent::Comment(_) => {}
            XmlEvent::Characters(_) => {}
            XmlEvent::Whitespace(_) => {}
            XmlEvent::EndDocument => {}
        }
    }
    Ok(graph)
}