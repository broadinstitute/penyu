use std::collections::BTreeMap;
use std::io::Write;
use crate::error::PenyuError;
use crate::model::graph::Graph;
use crate::model::node::{BlankNode, Entity, Node};
use crate::model::iri::Iri;
use crate::model::literal::Literal;

pub fn write<W: Write, G: Graph>(writer: &mut W, graph: &G) -> Result<(), PenyuError> {
    write_prefixes(writer, graph)?;
    write_triples(writer, graph)?;
    Ok(())
}

fn write_prefixes<W: Write, G: Graph>(writer: &mut W, graph: &G) -> Result<(), PenyuError> {
    for (key, iri) in graph.prefixes() {
        writeln!(writer, "PREFIX {}: <{}>", key, iri)?;
    }
    Ok(())
}

fn write_triples<W: Write, G: Graph>(writer: &mut W, graph: &G) -> Result<(), PenyuError> {
    let mut triples_iter = graph.triples();
    if let Some(triple1) = triples_iter.next() {
        write_entity(writer, &triple1.subject, graph.prefixes())?;
        write!(writer, " ")?;
        write_iri(writer, &triple1.predicate, graph.prefixes())?;
        write!(writer, " ")?;
        write_node(writer, &triple1.object, graph.prefixes())?;
        let mut last_triple = triple1;
        for triple in triples_iter {
            if triple.subject == last_triple.subject {
                if triple.predicate == last_triple.predicate {
                    write!(writer, ", ")?;
                    write_node(writer, &triple.object, graph.prefixes())?;
                } else {
                    writeln!(writer, ";\n    ")?;
                    write_iri(writer, &triple.predicate, graph.prefixes())?;
                    write!(writer, " ")?;
                    write_node(writer, &triple.object, graph.prefixes())?;
                }
            } else {
                writeln!(writer, " .")?;
                write_entity(writer, &triple.subject, graph.prefixes())?;
                write!(writer, " ")?;
                write_iri(writer, &triple.predicate, graph.prefixes())?;
                write!(writer, " ")?;
                write_node(writer, &triple.object, graph.prefixes())?;
            }
            last_triple = triple;
        }
        write!(writer, " .")?;
    }
    Ok(())
}

fn write_node<W: Write>(writer: &mut W, node: &Node, prefixes: &BTreeMap<String, Iri>)
                                  -> Result<(), PenyuError> {
    match node {
        Node::Entity(entity) => { write_entity(writer, entity, prefixes) }
        Node::Literal(literal) => { write_literal(writer, literal, prefixes) }
    }
}
fn write_entity<W: Write>(writer: &mut W, entity: &Entity,
                                    prefixes: &BTreeMap<String, Iri>) -> Result<(), PenyuError> {
    match entity {
        Entity::Iri(iri) => { write_iri(writer, iri, prefixes) }
        Entity::BlankNode(blank_node) => { write_blank_node(writer, blank_node) }
    }
}

fn write_iri<W: Write>(writer: &mut W, iri: &Iri, prefixes: &BTreeMap<String, Iri>)
                       -> Result<(), PenyuError> {
    let key_local =
        prefixes.iter().find_map(|(key, prefix_iri)| {
            iri.as_str().strip_prefix(prefix_iri.as_str()).map(|local| (key, local))
        });
    match key_local {
        None => { write!(writer, "<{}>", iri)? }
        Some((key, local)) => { write!(writer, "{}:{}", key, local)? }
    }
    Ok(())
}

fn write_blank_node<W: Write>(writer: &mut W, blank_node: &BlankNode) -> Result<(), PenyuError> {
    write!(writer, "_:{}", blank_node.id())?;
    Ok(())
}

fn write_literal<W: Write>(writer: &mut W, literal: &Literal, prefixes: &BTreeMap<String, Iri>)
    -> Result<(), PenyuError> {
    todo!()
}
