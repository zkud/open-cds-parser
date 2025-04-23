use crate::ast::*;

use super::super::Location;
use super::entity_term::EntityTerm;
use super::ActionDeclarationTerm;
use super::FunctionDeclarationTerm;
use super::IdentifierTerm;
use super::TypeDeclarationTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct ServiceTerm {
    #[prop]
    location: Location,
    #[subnode_prop]
    identifier: Box<IdentifierTerm>,
    #[subnode_prop]
    definitions: Vec<ServiceDefinition>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ServiceDefinition {
    Entity(EntityTerm),
    Function(FunctionDeclarationTerm),
    Action(ActionDeclarationTerm),
    Type(TypeDeclarationTerm),
}

impl Visitable for ServiceDefinition {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), V::Error> {
        match self {
            ServiceDefinition::Entity(entity) => entity.accept(visitor)?,
            ServiceDefinition::Function(function) => function.accept(visitor)?,
            ServiceDefinition::Action(action) => action.accept(visitor)?,
            ServiceDefinition::Type(type_declaration) => type_declaration.accept(visitor)?,
        };
        Ok(())
    }
}
