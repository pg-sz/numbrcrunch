use std::collections::HashMap;

pub enum Expression<'a> {
  Value(f64),
  Variable(&'a str),
  Add(Box<Expression<'a>>, Box<Expression<'a>>),
  Subtract(Box<Expression<'a>>, Box<Expression<'a>>),
  Multiply(Box<Expression<'a>>, Box<Expression<'a>>),
  Divide(Box<Expression<'a>>, Box<Expression<'a>>),
  Function(&'a str, &'a [Box<Expression<'a>>]),
}

pub trait Computable {
  /// computes a function value given a list of arguments
  fn compute(&self, arguments: &[Box<Expression>], context: &Context) -> f64;
}

struct Exponential {}

impl<'a> Exponential {
  fn get_expression(arguments: &'a [Box<Expression<'a>>]) -> Expression<'a> {
    Expression::Function("exp", arguments)
  }
}

impl Computable for Exponential {
  fn compute(&self, arguments: &[Box<Expression>], context: &Context) -> f64 {
    let expression =
      arguments.get(0).expect("exponential expects exactly one argument");
    return expression.eval(context).exp();
  }
}

type VariableRepository<'a> = HashMap<&'a str, f64>;
type FunctionRepository<'a> = HashMap<&'a str, &'a dyn Computable>;

pub struct Context<'a> {
  pub variable_repository: &'a mut VariableRepository<'a>,
  pub function_repository: &'a mut FunctionRepository<'a>,
}

trait Evaluable {
  fn eval(&self, context: &Context) -> f64;
}

impl<'a> Evaluable for Expression<'a> {
  fn eval(&self, context: &Context) -> f64
  {
    match self {
      Expression::Value(x) =>
        *x,
      Expression::Variable(variable_name) =>
        *context.variable_repository.get(variable_name).expect("variable unknown."),
      Expression::Add(lhs, rhs) =>
        lhs.eval(context) + rhs.eval(context),
      Expression::Subtract(lhs, rhs) =>
        lhs.eval(context) - rhs.eval(context),
      Expression::Multiply(lhs, rhs) =>
        lhs.eval(context) * rhs.eval(context),
      Expression::Divide(numerator, denominator) =>
        numerator.eval(context) / denominator.eval(context),
      Expression::Function(function_name, arguments) =>
        context.function_repository
          .get(function_name).expect("function unkown")
          .compute(arguments, context)
    }
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  use crate::{Context, Evaluable, Exponential, Expression, FunctionRepository, VariableRepository};

  #[test]
  fn it_value() {
    let context = Context {
      variable_repository: &mut VariableRepository::new(),
      function_repository: &mut FunctionRepository::new(),
    };
    let mock_value = 1.0;
    let value = Expression::Value(mock_value);
    assert_eq!(mock_value, value.eval(&context));
  }

  #[test]
  fn it_addition() {
    let context = Context {
      variable_repository: &mut VariableRepository::new(),
      function_repository: &mut FunctionRepository::new(),
    };
    let mock_values = [1.0, 2.0];
    let expected_result = 3.0;
    let value1 = Expression::Value(*mock_values.get(0).expect("not found"));
    let value2 = Expression::Value(*mock_values.get(1).expect("not found"));
    let result = Expression::Add(Box::new(value1), Box::new(value2));
    assert_eq!(expected_result, result.eval(&context));
  }

  #[test]
  fn it_subtract() {
    let context = Context {
      variable_repository: &mut VariableRepository::new(),
      function_repository: &mut FunctionRepository::new(),
    };
    let mock_values = [1.0, 2.0];
    let expected_result = -1.0;
    let value1 = Expression::Value(*mock_values.get(0).expect("not found"));
    let value2 = Expression::Value(*mock_values.get(1).expect("not found"));
    let result = Expression::Subtract(Box::new(value1), Box::new(value2));
    assert_eq!(expected_result, result.eval(&context));
  }

  #[test]
  fn it_multiply() {
    let context = Context {
      variable_repository: &mut VariableRepository::new(),
      function_repository: &mut FunctionRepository::new(),
    };
    let mock_values = [2.0, 3.0];
    let expected_result = 6.0;
    let value1 = Expression::Value(*mock_values.get(0).expect("not found"));
    let value2 = Expression::Value(*mock_values.get(1).expect("not found"));
    let result = Expression::Multiply(Box::new(value1), Box::new(value2));
    assert_eq!(expected_result, result.eval(&context));
  }

  #[test]
  fn it_divide() {
    let context = Context {
      variable_repository: &mut VariableRepository::new(),
      function_repository: &mut FunctionRepository::new(),
    };
    let mock_values = [3.0, 2.0];
    let expected_result = 1.5;
    let value1 = Expression::Value(*mock_values.get(0).expect("not found"));
    let value2 = Expression::Value(*mock_values.get(1).expect("not found"));
    let result = Expression::Divide(Box::new(value1), Box::new(value2));
    assert_eq!(expected_result, result.eval(&context));
  }

  #[test]
  fn it_test_repository() {
    let context = Context {
      variable_repository: &mut VariableRepository::new(),
      function_repository: &mut FunctionRepository::new(),
    };
    context.variable_repository.insert("x", 1.0);

    let result = Expression::Variable("x");

    assert_eq!(1.0, result.eval(&context));
  }

  #[test]
  fn it_test_exponential() {
    let exponential = Exponential {};

    let context = Context {
      variable_repository: &mut VariableRepository::new(),
      function_repository: &mut FunctionRepository::new(),
    };

    context.function_repository.insert("exp", &exponential);

    let variable = [Box::new(Expression::Value(0.0))];
    let exponential = Exponential::get_expression(&variable);

    assert_eq!(1.0, exponential.eval(&context));

    let variable_one = [Box::new(Expression::Value(1.0))];
    let exponential_euler = Exponential::get_expression(&variable_one);

    assert_eq!(std::f64::consts::E, exponential_euler.eval(&context));
  }
}
