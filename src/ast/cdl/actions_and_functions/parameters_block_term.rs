use crate::ast::Visitable;

use super::super::super::PunctuationSignTerm;
use super::super::ParamTerm;
use crate::ast::Location;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct ParametersBlockTerm {
    #[prop]
    location: Location,
    #[subnode_prop]
    open_bracket: Box<PunctuationSignTerm>,
    #[subnode_prop]
    parameters_and_commas: Vec<ParameterOrComma>,
    #[subnode_prop]
    close_bracket: Box<PunctuationSignTerm>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ParameterOrComma {
    Parameter(ParamTerm),
    Comma(PunctuationSignTerm),
}

impl Visitable for ParameterOrComma {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), V::Error> {
        match self {
            Self::Parameter(parameter) => parameter.accept(visitor)?,
            Self::Comma(comma) => comma.accept(visitor)?,
        };
        Ok(())
    }
}
