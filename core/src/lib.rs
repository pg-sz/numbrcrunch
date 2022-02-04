

enum Expression {
    Value(f64),
    Variable(String),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Function(String, Box<[Expression]>)
}

trait Eval {
    fn eval(&self) -> f64;
}

impl Eval for Expression {
    fn eval(&self) -> f64 {
        match self {
            Expression::Value(x) => *x,
            Expression::Variable(_) => 0.0, // TODO:
            Expression::Add(lhs, rhs) => lhs.eval() + rhs.eval(),
            Expression::Subtract(lhs, rhs) => lhs.eval() - rhs.eval(),
            Expression::Multiply(lhs, rhs) => lhs.eval() * rhs.eval(),
            Expression::Divide(numerator, denominator) => numerator.eval() / denominator.eval(),
            Expression::Function(_, _) => 0.0, // TODO: plan function mapper
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Eval, Expression};

    #[test]
    fn it_value() {
        let mock_value = 1.0;
        let value = Expression::Value(mock_value);
        assert_eq!(mock_value, value.eval());
    }

    #[test]
    fn it_addition() {
        let mock_values = [1.0, 2.0];
        let expected_result = 3.0;
        let value1 = Expression::Value(*mock_values.get(0).expect("not found"));
        let value2 = Expression::Value(*mock_values.get(1).expect("not found"));
        let addition = Expression::Add(Box::new(value1), Box::new(value2));
        assert_eq!(expected_result, addition.eval());
    }

    #[test]
    fn it_subtract() {
        let mock_values = [1.0, 2.0];
        let expected_result = -1.0;
        let value1 = Expression::Value(*mock_values.get(0).expect("not found"));
        let value2 = Expression::Value(*mock_values.get(1).expect("not found"));
        let addition = Expression::Subtract(Box::new(value1), Box::new(value2));
        assert_eq!(expected_result, addition.eval());
    }

    #[test]
    fn it_multiply() {
        let mock_values = [2.0, 3.0];
        let expected_result = 6.0;
        let value1 = Expression::Value(*mock_values.get(0).expect("not found"));
        let value2 = Expression::Value(*mock_values.get(1).expect("not found"));
        let addition = Expression::Multiply(Box::new(value1), Box::new(value2));
        assert_eq!(expected_result, addition.eval());
    }

    #[test]
    fn it_divide() {
        let mock_values = [3.0, 2.0];
        let expected_result = 1.5;
        let value1 = Expression::Value(*mock_values.get(0).expect("not found"));
        let value2 = Expression::Value(*mock_values.get(1).expect("not found"));
        let addition = Expression::Divide(Box::new(value1), Box::new(value2));
        assert_eq!(expected_result, addition.eval());
    }
}
