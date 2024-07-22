use super::Checker;
use crate::ast::ast;
use crate::diagnostics::Diagnostic;
use crate::diagnostics::TypeError;
use crate::types::Type;

impl Checker {
  pub fn check_identifier_expression(&mut self, identifier: &ast::IdentifierExpression) -> Result<Type, Diagnostic> {
    let text_name = identifier.name.clone();
    if !self.ctx.is_variable_defined(text_name.as_str()) {
      return Err(self.create_diagnostic(TypeError::UndeclaredVariable(
        text_name.to_string(),
        // TODO: hei :), use name location or a value location?
        Some(identifier.location.clone()),
      )));
    }
    self.ctx.use_variable(text_name.as_str());
    let type_ = self.ctx.get_variable(text_name.as_str()).unwrap().clone();
    Ok(type_)
  }
}
