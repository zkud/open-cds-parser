use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct OpenRoundBracketTerm {
    location: Location,
}
