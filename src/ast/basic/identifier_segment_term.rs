use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct IdentifierSegmentTerm {
    #[prop]
    value: String,
}
