use super::super::super::KeywordTerm;
use super::super::TypeReferenceTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct ReturnsDeclarationTerm {
    #[prop]
    location: Location,
    #[subnode_prop]
    returns: Box<KeywordTerm>,
    #[subnode_prop]
    type_reference: Box<TypeReferenceTerm>,
}
