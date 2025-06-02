use crate::ast::*;

use crate::ast::ActionDeclarationTerm;
use crate::ast::EntityDeclarationTerm;
use crate::ast::FunctionDeclarationTerm;
use crate::ast::IdentifierTerm;
use crate::ast::Location;
use crate::ast::TypeDeclarationTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct ServiceDeclarationTerm {
    #[prop]
    location: Location,
    #[subnode_prop]
    service: Box<KeywordTerm>,
    #[subnode_prop]
    identifier: Box<IdentifierTerm>,
    #[subnode_prop]
    open_brace: Box<PunctuationSignTerm>,
    #[subnode_prop]
    definitions: Vec<ServiceDefinition>,
    #[subnode_prop]
    close_brace: Box<PunctuationSignTerm>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ServiceDefinition {
    Entity(EntityDeclarationTerm),
    Function(FunctionDeclarationTerm),
    Action(ActionDeclarationTerm),
    Type(TypeDeclarationTerm),
}

impl Visitable for ServiceDefinition {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), V::Error> {
        match self {
            ServiceDefinition::Entity(entity_declaration) => entity_declaration.accept(visitor)?,
            ServiceDefinition::Function(function) => function.accept(visitor)?,
            ServiceDefinition::Action(action) => action.accept(visitor)?,
            ServiceDefinition::Type(type_declaration) => type_declaration.accept(visitor)?,
        };
        Ok(())
    }
}
