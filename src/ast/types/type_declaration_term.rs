use crate::ast::KeywordTerm;
use crate::ast::PunctuationSignTerm;

use super::super::IdentifierTerm;
use super::super::Location;
use super::TypeReferenceTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct TypeDeclarationTerm {
    #[prop]
    location: Location,
    #[subnode_prop]
    define: Option<Box<KeywordTerm>>,
    #[subnode_prop]
    type_keyword: Box<KeywordTerm>,
    #[subnode_prop]
    identifier: Box<IdentifierTerm>,
    #[subnode_prop]
    colon: Option<Box<PunctuationSignTerm>>,
    #[subnode_prop]
    resolved_type: Box<TypeReferenceTerm>,
    #[subnode_prop]
    semicolumn: Box<PunctuationSignTerm>,
}
