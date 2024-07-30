use super::Checker;
use crate::ast::ast;
use crate::diagnostics::{Diagnostic, TypeError};
use crate::types::Type;

impl Checker<'_> {
  pub fn check_assign_statement(&mut self, assign: &ast::AssignStatement) -> Result<Type, Diagnostic> {
    let right_t = self.check_expression(&assign.value)?;
    let value_location = assign.value.get_location();
    let lexema = assign.name.lexeme();
    let (defined, scope_idx) = self.ctx.defined_in_any_scope(lexema);

    if !defined {
      let diagnostic = TypeError::UndeclaredVariable(lexema.to_string(), Some(assign.name.location.clone()));
      return Err(self.create_diagnostic(diagnostic));
    }

    let left_t = self.ctx.get_variable_in_scope(lexema, scope_idx).unwrap().clone();

    if left_t.check_match(&right_t) {
      let location = Some(assign.location.clone());
      let diagnostic = TypeError::TypeMismatchAssignment(left_t.to_string(), right_t.to_string(), Some(value_location));
      return Err(self.create_diagnostic(diagnostic));
    }

    if right_t.can_replace(&left_t) {
      let ok = self.ctx.redeclare_variable(lexema, right_t.clone());
      return Ok(right_t);
    }
    Ok(left_t)
  }
}
