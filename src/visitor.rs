use super::ast::action_term::ActionTerm;
use super::ast::entity_term::EntityTerm;
use super::ast::field_term::FieldTerm;
use super::ast::function_term::FunctionTerm;
use super::ast::module_term::ModuleTerm;
use super::ast::name_term::NameTerm;
use super::ast::param_term::ParamTerm;
use super::ast::returns_term::ReturnsTerm;
use super::ast::service_term::ServiceTerm;
use super::ast::type_term::TypeTerm;
use super::visitor_error::VisitorError;

pub trait Visitor {
  fn process_name(&mut self, term: &NameTerm) -> Result<(), VisitorError> {
    Ok(())
  }
  fn process_type(&mut self, term: &TypeTerm) -> Result<(), VisitorError> {
    Ok(())
  }
  fn process_param(&mut self, term: &ParamTerm) -> Result<(), VisitorError> {
    Ok(())
  }
  fn process_returns(&mut self, term: &ReturnsTerm) -> Result<(), VisitorError> {
    Ok(())
  }
  fn process_field(&mut self, term: &FieldTerm) -> Result<(), VisitorError> {
    Ok(())
  }
  fn process_function(&mut self, term: &FunctionTerm) -> Result<(), VisitorError> {
    Ok(())
  }
  fn process_action(&mut self, term: &ActionTerm) -> Result<(), VisitorError> {
    Ok(())
  }
  fn process_entity(&mut self, term: &EntityTerm) -> Result<(), VisitorError> {
    Ok(())
  }
  fn process_service(&mut self, term: &ServiceTerm) -> Result<(), VisitorError> {
    Ok(())
  }
  fn process_module(&mut self, term: &ModuleTerm) -> Result<(), VisitorError> {
    Ok(())
  }
}
