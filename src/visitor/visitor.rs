use super::super::ast::*;
#[cfg(test)]
use super::super::ast::{LeafTerm, NodeTerm};

pub trait Visitor<E> {
    fn process_name(&mut self, _term: &NameTerm) -> Result<(), E> {
        Ok(())
    }
    fn process_type(&mut self, _term: &TypeTerm) -> Result<(), E> {
        Ok(())
    }
    fn process_param(&mut self, _term: &ParamTerm) -> Result<(), E> {
        Ok(())
    }
    fn process_returns(&mut self, _term: &ReturnsTerm) -> Result<(), E> {
        Ok(())
    }
    fn process_field(&mut self, _term: &FieldTerm) -> Result<(), E> {
        Ok(())
    }
    fn process_function(&mut self, _term: &FunctionTerm) -> Result<(), E> {
        Ok(())
    }
    fn process_action(&mut self, _term: &ActionTerm) -> Result<(), E> {
        Ok(())
    }
    fn process_entity(&mut self, _term: &EntityTerm) -> Result<(), E> {
        Ok(())
    }
    fn process_service(&mut self, _term: &ServiceTerm) -> Result<(), E> {
        Ok(())
    }
    fn process_module(&mut self, _term: &ModuleTerm) -> Result<(), E> {
        Ok(())
    }
    fn process_path(&mut self, _term: &PathTerm) -> Result<(), E> {
        Ok(())
    }
    fn process_import(&mut self, _term: &ImportTerm) -> Result<(), E> {
        Ok(())
    }
    fn process_selector(&mut self, _term: &SelectorTerm) -> Result<(), E> {
        Ok(())
    }
    fn process_selection_block(&mut self, _term: &SelectionBlockTerm) -> Result<(), E> {
        Ok(())
    }
    fn process_select_all(&mut self, _term: &SelectAllTerm) -> Result<(), E> {
        Ok(())
    }
    fn process_dot(&mut self, _term: &DotTerm) -> Result<(), E> {
        Ok(())
    }
    fn process_wildcart(&mut self, _term: &WildcartTerm) -> Result<(), E> {
        Ok(())
    }
    fn process_import_identifier(&mut self, _term: &ImportIdentifierTerm) -> Result<(), E> {
        Ok(())
    }
    fn process_comma(&mut self, _term: &CommaTerm) -> Result<(), E> {
        Ok(())
    }
    fn process_as(&mut self, _term: &AsTerm) -> Result<(), E> {
        Ok(())
    }
    fn process_alias(&mut self, _term: &AliasTerm) -> Result<(), E> {
        Ok(())
    }
    fn process_from(&mut self, _term: &FromTerm) -> Result<(), E> {
        Ok(())
    }
    fn process_semicolumn(&mut self, _term: &SemicolumnTerm) -> Result<(), E> {
        Ok(())
    }
    fn process_open_curly_brace(&mut self, _term: &OpenCurlyBraceTerm) -> Result<(), E> {
        Ok(())
    }
    fn process_close_curly_brace(&mut self, _term: &CloseCurlyBraceTerm) -> Result<(), E> {
        Ok(())
    }
    fn process_using(&mut self, _term: &UsingTerm) -> Result<(), E> {
        Ok(())
    }
    #[cfg(test)]
    fn process_mock_leaf(&mut self, _term: &LeafTerm) -> Result<(), E> {
        Ok(())
    }
    #[cfg(test)]
    fn process_mock_node(&mut self, _term: &NodeTerm) -> Result<(), E> {
        Ok(())
    }
}
