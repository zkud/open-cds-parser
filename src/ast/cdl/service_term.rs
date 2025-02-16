use crate::ast::*;

use super::entity_term::EntityTerm;
use super::function_term::FunctionTerm;
use super::name_term::NameTerm;
use super::ActionDeclarationTerm;
use super::TypeDeclarationTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct ServiceTerm {
    #[subnode_prop]
    name: Box<NameTerm>,
    #[subnode_prop]
    definitions: Vec<ServiceDefinition>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ServiceDefinition {
    Entity(EntityTerm),
    Function(FunctionTerm),
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
