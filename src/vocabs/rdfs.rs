use crate::model::iri::Iri;

pub const PREFIX: &str = "rdfs";
pub const NAMESPACE: &Iri = &Iri::new_str("http://www.w3.org/2000/01/rdf-schema#");
pub const RESOURCE: &Iri = &NAMESPACE.join_str("Resource");
pub const CLASS: &Iri = &NAMESPACE.join_str("Class");
pub const LITERAL: &Iri = &NAMESPACE.join_str("Literal");
pub const DATATYPE: &Iri = &NAMESPACE.join_str("Datatype");
pub const CONTAINER: &Iri = &NAMESPACE.join_str("Container");
pub const CONTAINER_MEMBERSHIP_PROPERTY: &Iri =
    &NAMESPACE.join_str("ContainerMembershipProperty");
pub const SUB_CLASS_OF: &Iri = &NAMESPACE.join_str("subClassOf");
pub const SUB_PROPERTY_OF: &Iri = &NAMESPACE.join_str("subPropertyOf");
pub const DOMAIN: &Iri = &NAMESPACE.join_str("domain");
pub const RANGE: &Iri = &NAMESPACE.join_str("range");
pub const LABEL: &Iri = &NAMESPACE.join_str("label");
pub const COMMENT: &Iri = &NAMESPACE.join_str("comment");
pub const MEMBER: &Iri = &NAMESPACE.join_str("member");
pub const IS_DEFINED_BY: &Iri = &NAMESPACE.join_str("isDefinedBy");
pub const SEE_ALSO: &Iri = &NAMESPACE.join_str("seeAlso");
