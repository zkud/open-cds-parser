use crate::ast::*;

use super::super::import::ImportTerm;
use super::service_term::ServiceTerm;
use super::EntityDeclarationTerm;
use super::TypeDeclarationTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct ModuleTerm {
    #[prop]
    location: Location,
    #[subnode_prop]
    definitions: Vec<ModuleDefinition>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ModuleDefinition {
    Entity(EntityDeclarationTerm),
    Type(TypeDeclarationTerm),
    Service(ServiceTerm),
    Import(ImportTerm),
}

impl Visitable for ModuleDefinition {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), V::Error> {
        match self {
            ModuleDefinition::Entity(entity_declaration) => entity_declaration.accept(visitor)?,
            ModuleDefinition::Type(type_declaration) => type_declaration.accept(visitor)?,
            ModuleDefinition::Service(service) => service.accept(visitor)?,
            ModuleDefinition::Import(import) => import.accept(visitor)?,
        };
        Ok(())
    }
}
