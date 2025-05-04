use ast_term_derive::ASTTerm;

/// Aggregate term for all punctuation sign, like "{", "}", ";" an so on
#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct PunctuationSignTerm {
    #[prop]
    location: Location,
    #[prop]
    sign: PunctuationSign,
}

/// All possible CDS keywords
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum PunctuationSign {
    OpenCurlyBrace,    // }
    CloseCurlyBrace,   // {
    OpenRoundBracket,  // (
    CloseRoundBracket, // )
    Colon,             // :
    Comma,             // ,
    Dot,               // .
    Semicolumn,        // ;
    Wildcart,          // *
}
