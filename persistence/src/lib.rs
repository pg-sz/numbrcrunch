use serde::{Deserialize, Serialize};

use core::Expression;

#[derive(Serialize, Deserialize)]
#[serde(remote = "Expression")]
enum ExpressionDef<'a> {
  Value(f64),
  Variable(&'a str),
  Add(Box<ExpressionDef<'a>>, Box<ExpressionDef<'a>>),
  Subtract(Box<ExpressionDef<'a>>, Box<ExpressionDef<'a>>),
  Multiply(Box<ExpressionDef<'a>>, Box<ExpressionDef<'a>>),
  Divide(Box<ExpressionDef<'a>>, Box<ExpressionDef<'a>>),
  Function(&'a str, &'a [Box<ExpressionDef<'a>>]),
}

impl From<ExpressionDef> for Expression {
  fn from(def: ExpressionDef) -> Expression {
    match def {
      ExpressionDef::Value(value) => Expression::Value(value),
      ExpressionDef::Variable(variable_name) => Expression::Variable(variable_name),
      ExpressionDef::Add(lhs, rhs) =>
        Expression::Add(Box::new(Expression::from(*lhs)),
                        Box::new(Expression::from(*rhs))),
      ExpressionDef::Subtract(lhs, rhs) =>
        Expression::Subtract(Box::new(Expression::from(*lhs)),
                             Box::new(Expression::from(*rhs))),
      ExpressionDef::Multiply(lhs, rhs) =>
        Expression::Multiply(Box::new(Expression::from(*lhs)),
                             Box::new(Expression::from(*rhs))),
      ExpressionDef::Divide(lhs, rhs) =>
        Expression::Divide(Box::new(Expression::from(*lhs)),
                           Box::new(Expression::from(*rhs))),
      ExpressionDef::Function(function_name, arguments) => {
        let test: [Box<Expression>] = arguments.iter().map(|argument| Expression::from(**argument)).collect();
        Expression::Function(function_name, &test)
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use serde;

  #[test]
  fn it_works() {
    let s = serde_yaml::to_string(&value).expect("panic");
    assert_eq!(s, "---\nQueen:\n  - Jake:\n      - 1\n      - 2.0\n  - King: false\n");
  }
}
