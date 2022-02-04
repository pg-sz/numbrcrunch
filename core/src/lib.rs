use std::collections::{HashMap};
use std::env::var;

type VariableRepository<'a> = HashMap<&'a str, f64>;

enum Expression {
    Value(f64),
    Variable(String),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Exponential(Box<Expression>)
}

trait Evaluable {
    fn eval(&self, variable_repository: &VariableRepository) -> f64;
}

impl Evaluable for Expression {
    fn eval(&self, variable_repository: &VariableRepository) -> f64 {
        match self {
            Expression::Value(x) => *x,
            Expression::Variable(name) => *variable_repository.get(name.as_str()).expect("variable unknown."),
            Expression::Add(lhs, rhs) => lhs.eval(variable_repository) + rhs.eval(variable_repository),
            Expression::Subtract(lhs, rhs) => lhs.eval(variable_repository) - rhs.eval(variable_repository),
            Expression::Multiply(lhs, rhs) => lhs.eval(variable_repository) * rhs.eval(variable_repository),
            Expression::Divide(numerator, denominator) => numerator.eval(variable_repository) / denominator.eval(variable_repository),
            Expression::Exponential(argument) => argument.eval(variable_repository).exp()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::{Evaluable, Expression, VariableRepository};

    #[test]
    fn it_value() {
        let repository: VariableRepository = VariableRepository::new();
        let mock_value = 1.0;
        let value = Expression::Value(mock_value);
        assert_eq!(mock_value, value.eval(&repository));
    }

    #[test]
    fn it_addition() {
        let repository: VariableRepository = VariableRepository::new();
        let mock_values = [1.0, 2.0];
        let expected_result = 3.0;
        let value1 = Expression::Value(*mock_values.get(0).expect("not found"));
        let value2 = Expression::Value(*mock_values.get(1).expect("not found"));
        let result = Expression::Add(Box::new(value1), Box::new(value2));
        assert_eq!(expected_result, result.eval(&repository));
    }

    #[test]
    fn it_subtract() {
        let repository: VariableRepository = VariableRepository::new();
        let mock_values = [1.0, 2.0];
        let expected_result = -1.0;
        let value1 = Expression::Value(*mock_values.get(0).expect("not found"));
        let value2 = Expression::Value(*mock_values.get(1).expect("not found"));
        let result = Expression::Subtract(Box::new(value1), Box::new(value2));
        assert_eq!(expected_result, result.eval(&repository));
    }

    #[test]
    fn it_multiply() {
        let repository: VariableRepository = VariableRepository::new();
        let mock_values = [2.0, 3.0];
        let expected_result = 6.0;
        let value1 = Expression::Value(*mock_values.get(0).expect("not found"));
        let value2 = Expression::Value(*mock_values.get(1).expect("not found"));
        let result = Expression::Multiply(Box::new(value1), Box::new(value2));
        assert_eq!(expected_result, result.eval(&repository));
    }

    #[test]
    fn it_divide() {
        let repository: VariableRepository = VariableRepository::new();
        let mock_values = [3.0, 2.0];
        let expected_result = 1.5;
        let value1 = Expression::Value(*mock_values.get(0).expect("not found"));
        let value2 = Expression::Value(*mock_values.get(1).expect("not found"));
        let result = Expression::Divide(Box::new(value1), Box::new(value2));
        assert_eq!(expected_result, result.eval(&repository));
    }

    #[test]
    fn it_test_repository() {
        let mut repository: VariableRepository = VariableRepository::new();
        repository.insert("x", 1.0);

        let result = Expression::Variable("x".to_string());

        assert_eq!(1.0, result.eval(&repository));
    }

    #[test]
    fn it_test_exponential() {
        let repository: VariableRepository = VariableRepository::new();
        let variable = Expression::Value(0.0);

        assert_eq!(1.0, Expression::Exponential(Box::new(variable)).eval(&repository));
    }
}
