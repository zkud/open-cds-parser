use crate::ast::*;

use super::super::import::ImportTerm;
use super::entity_term::EntityTerm;
use super::service_term::ServiceTerm;
use super::TypeDeclarationTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct ModuleTerm {
    #[subnode_prop]
    definitions: Vec<ModuleDefinition>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ModuleDefinition {
    Entity(EntityTerm),
    Type(TypeDeclarationTerm),
    Service(ServiceTerm),
    Import(ImportTerm),
}

impl Visitable for ModuleDefinition {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), V::Error> {
        match self {
            ModuleDefinition::Entity(entity) => entity.accept(visitor)?,
            ModuleDefinition::Type(type_declaration) => type_declaration.accept(visitor)?,
            ModuleDefinition::Service(service) => service.accept(visitor)?,
            ModuleDefinition::Import(import) => import.accept(visitor)?,
        };
        Ok(())
    }
}
