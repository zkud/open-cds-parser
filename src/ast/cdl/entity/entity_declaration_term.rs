use crate::ast::IdentifierTerm;
use crate::ast::KeywordTerm;
use crate::ast::Location;
use crate::ast::PunctuationSignTerm;
use crate::ast::StructureTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct EntityDeclarationTerm {
    #[prop]
    location: Location,
    #[subnode_prop]
    define: Option<Box<KeywordTerm>>,
    #[subnode_prop]
    entity: Box<KeywordTerm>,
    #[subnode_prop]
    identifier: Box<IdentifierTerm>,
    #[subnode_prop]
    colon: Option<Box<PunctuationSignTerm>>,
    #[subnode_prop]
    applied_aspects: Vec<IdentifierTerm>,
    #[subnode_prop]
    structure: Box<StructureTerm>,
    #[subnode_prop]
    semicolumn: Option<Box<PunctuationSignTerm>>,
}
