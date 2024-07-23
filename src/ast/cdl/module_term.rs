use super::super::super::visitor::{Visitor, VisitorError};
use super::super::common::ast_term::ASTTerm;
use super::entity_term::EntityTerm;
use super::service_term::ServiceTerm;
use super::type_term::TypeTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug)]
#[ast_term(visitor_path = "process_module")]
pub struct ModuleTerm {
    #[subnode_prop]
    definitions: Vec<ModuleDefinition>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum ModuleDefinition {
    Entity(EntityTerm),
    Type(TypeTerm),
    Service(ServiceTerm),
}

impl ASTTerm for ModuleDefinition {
    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
        match self {
            ModuleDefinition::Entity(entity) => entity.accept(visitor)?,
            ModuleDefinition::Type(type_declaration) => type_declaration.accept(visitor)?,
            ModuleDefinition::Service(service) => service.accept(visitor)?,
        };
        Ok(())
    }
}
