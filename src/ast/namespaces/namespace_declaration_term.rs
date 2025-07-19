use crate::ast::KeywordTerm;
use crate::ast::PunctuationSignTerm;

use super::super::IdentifierTerm;
use super::super::Location;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct NamespaceDeclarationTerm {
    #[prop]
    location: Location,
    #[subnode_prop]
    namespace: Box<KeywordTerm>,
    #[subnode_prop]
    identifier: Box<IdentifierTerm>,
    #[subnode_prop]
    semicolumn: Box<PunctuationSignTerm>,
}
