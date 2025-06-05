

use exercise4::type_checker::{TypeChecker, Type};
use exercise4::expression::Expression;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true() {
        let checker = TypeChecker;
        let expr = Expression::True;
        assert_eq!(checker.visit(&expr), Ok(Type::Bool));
    }

    #[test]
    fn test_negation_of_number() {
        let checker = TypeChecker;
        let expr = Expression::Neg(Box::new(Expression::Num(5)));
        assert_eq!(checker.visit(&expr), Ok(Type::Num));
    }

}