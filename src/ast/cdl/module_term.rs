use super::super::super::visitor::Visitor;
use super::super::common::ast_term::ASTTerm;
use super::entity_term::EntityTerm;
use super::import_term::ImportTerm;
use super::service_term::ServiceTerm;
use super::type_term::TypeTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
#[ast_term(visitor_path = "process_module")]
pub struct ModuleTerm {
    #[subnode_prop]
    definitions: Vec<ModuleDefinition>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ModuleDefinition {
    Entity(EntityTerm),
    Type(TypeTerm),
    Service(ServiceTerm),
    Import(ImportTerm),
}

impl ASTTerm for ModuleDefinition {
    fn accept<E>(&self, visitor: &mut dyn Visitor<E>) -> Result<(), E> {
        match self {
            ModuleDefinition::Entity(entity) => entity.accept(visitor)?,
            ModuleDefinition::Type(type_declaration) => type_declaration.accept(visitor)?,
            ModuleDefinition::Service(service) => service.accept(visitor)?,
            ModuleDefinition::Import(import) => import.accept(visitor)?,
        };
        Ok(())
    }
}
