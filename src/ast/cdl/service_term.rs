use super::super::super::visitor::Visitor;
use super::super::common::ast_term::ASTTerm;
use super::action_term::ActionTerm;
use super::entity_term::EntityTerm;
use super::function_term::FunctionTerm;
use super::name_term::NameTerm;
use super::type_term::TypeTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
#[ast_term(visitor_path = "process_service")]
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
    Action(ActionTerm),
    Type(TypeTerm),
}

impl ASTTerm for ServiceDefinition {
    fn accept<E>(&self, visitor: &mut dyn Visitor<E>) -> Result<(), E> {
        match self {
            ServiceDefinition::Entity(entity) => entity.accept(visitor)?,
            ServiceDefinition::Function(function) => function.accept(visitor)?,
            ServiceDefinition::Action(action) => action.accept(visitor)?,
            ServiceDefinition::Type(type_declaration) => type_declaration.accept(visitor)?,
        };
        Ok(())
    }
}
