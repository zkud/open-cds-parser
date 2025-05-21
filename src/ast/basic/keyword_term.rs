use ast_term_derive::ASTTerm;

/// Aggregate term for all keywords, like function, action, entity an so on
#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct KeywordTerm {
    #[prop]
    location: Location,
    #[prop]
    keyword: Keyword,
}

/// All possible CDS keywords
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Keyword {
    As,       // as
    Using,    // using
    Action,   // action
    Returns,  // returns
    Function, // function
    Array,    // array
    Many,     // many
    Of,       // of
    From,     // from
    Define,   // define
    Type,     // type
    Entity,   // entity
}
