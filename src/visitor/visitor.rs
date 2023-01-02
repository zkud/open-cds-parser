use super::super::ast::{
  ActionTerm, EntityTerm, FieldTerm, FilePathTerm, FunctionTerm, ModuleTerm, NameTerm, ParamTerm,
  ReturnsTerm, RootTerm, ServiceTerm, TypeTerm, UsingRefTerm, UsingTerm,
};
use super::VisitorError;

pub trait Visitor {
  fn process_root(&mut self, _term: &RootTerm) -> Result<(), VisitorError> {
    Ok(())
  }
  fn process_using(&mut self, _term: &UsingTerm) -> Result<(), VisitorError> {
    Ok(())
  }
  fn process_using_ref(&mut self, _term: &UsingRefTerm) -> Result<(), VisitorError> {
    Ok(())
  }
  fn process_file_path(&mut self, _term: &FilePathTerm) -> Result<(), VisitorError> {
    Ok(())
  }
  fn process_name(&mut self, _term: &NameTerm) -> Result<(), VisitorError> {
    Ok(())
  }
  fn process_type(&mut self, _term: &TypeTerm) -> Result<(), VisitorError> {
    Ok(())
  }
  fn process_param(&mut self, _term: &ParamTerm) -> Result<(), VisitorError> {
    Ok(())
  }
  fn process_returns(&mut self, _term: &ReturnsTerm) -> Result<(), VisitorError> {
    Ok(())
  }
  fn process_field(&mut self, _term: &FieldTerm) -> Result<(), VisitorError> {
    Ok(())
  }
  fn process_function(&mut self, _term: &FunctionTerm) -> Result<(), VisitorError> {
    Ok(())
  }
  fn process_action(&mut self, _term: &ActionTerm) -> Result<(), VisitorError> {
    Ok(())
  }
  fn process_entity(&mut self, _term: &EntityTerm) -> Result<(), VisitorError> {
    Ok(())
  }
  fn process_service(&mut self, _term: &ServiceTerm) -> Result<(), VisitorError> {
    Ok(())
  }
  fn process_module(&mut self, _term: &ModuleTerm) -> Result<(), VisitorError> {
    Ok(())
  }
}
