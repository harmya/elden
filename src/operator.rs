use crate::utils::extract_operator_and_delimiter;

#[derive(Debug, PartialEq)]
pub enum Operator {
    Arithmetic(ArithmeticOperator),
    Relational(RelationalOperator),
    Logical(LogicalOperator),
    Assignment(AssignmentOperator),
}

#[derive(Debug, PartialEq)]
pub enum ArithmeticOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, PartialEq)]
pub enum RelationalOperator {
    BangEquals,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

#[derive(Debug, PartialEq)]
pub enum LogicalOperator {
    And,
    Or,
    Not,
}

#[derive(Debug, PartialEq)]
pub enum AssignmentOperator {
    Equal,
}

impl Operator {
    pub fn new(s: &str) -> Result<(Self, &str), String> {
        let (operator, rest) = match extract_operator_and_delimiter(s.trim()) {
            Ok((operator, rest)) => (operator, rest),
            Err(err) => panic!("{}", err),
        };
        if let Ok((arithmetic_op, rest)) = ArithmeticOperator::try_new(operator, rest) {
            return Ok((Self::Arithmetic(arithmetic_op), rest));
        }
        if let Ok((relational_op, rest)) = RelationalOperator::try_new(operator, rest) {
            return Ok((Self::Relational(relational_op), rest));
        }
        if let Ok((logical_op, rest)) = LogicalOperator::try_new(operator, rest) {
            return Ok((Self::Logical(logical_op), rest));
        }
        if let Ok((assignment_op, rest)) = AssignmentOperator::try_new(operator, rest) {
            return Ok((Self::Assignment(assignment_op), rest));
        }
        return Err(format!("Illegal Operator: {}", operator));
    }
}

impl ArithmeticOperator {
    pub fn try_new<'a>(operator: &str, rest: &'a str) -> Result<(Self, &'a str), String> {
        let op = match operator {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            "%" => Self::Mod,
            _ => return Err(format!("Illegal Arithmetic Operator: {}", operator)),
        };
        Ok((op, rest))
    }
}

impl RelationalOperator {
    pub fn try_new<'a>(operator: &str, rest: &'a str) -> Result<(Self, &'a str), String> {
        let op = match operator {
            "!=" => Self::BangEquals,
            "==" => Self::EqualEqual,
            ">" => Self::Greater,
            ">=" => Self::GreaterEqual,
            "<" => Self::Less,
            "<=" => Self::LessEqual,
            _ => return Err(format!("Illegal Relational Operator: {}", operator)),
        };
        Ok((op, rest))
    }
}

impl LogicalOperator {
    pub fn try_new<'a>(operator: &str, rest: &'a str) -> Result<(Self, &'a str), String> {
        let op = match operator {
            "!" => Self::Not,
            "||" => Self::Or,
            "&&" => Self::And,
            _ => return Err(format!("Illegal Logical Operator: {}", operator)),
        };
        Ok((op, rest))
    }
}

impl AssignmentOperator {
    pub fn try_new<'a>(operator: &str, rest: &'a str) -> Result<(Self, &'a str), String> {
        let op = match operator {
            "=" => Self::Equal,
            _ => return Err(format!("Illegal Assignment Operator: {}", operator)),
        };
        Ok((op, rest))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_add_with_whitespace() {
        assert_eq!(
            Operator::new("  +  "),
            Ok((Operator::Arithmetic(ArithmeticOperator::Add), ""))
        );
    }

    #[test]
    fn parse_subtract_with_whitespace() {
        assert_eq!(
            Operator::new("  -  "),
            Ok((Operator::Arithmetic(ArithmeticOperator::Sub), ""))
        );
    }

    #[test]
    fn parse_multiply_with_whitespace() {
        assert_eq!(
            Operator::new("  *  "),
            Ok((Operator::Arithmetic(ArithmeticOperator::Mul), ""))
        );
    }

    #[test]
    fn parse_divide_with_whitespace() {
        assert_eq!(
            Operator::new("  /  "),
            Ok((Operator::Arithmetic(ArithmeticOperator::Div), ""))
        );
    }

    #[test]
    fn parse_modulus_with_whitespace() {
        assert_eq!(
            Operator::new("  %  "),
            Ok((Operator::Arithmetic(ArithmeticOperator::Mod), ""))
        );
    }

    #[test]
    fn parse_add_without_whitespace() {
        assert_eq!(
            Operator::new("+"),
            Ok((Operator::Arithmetic(ArithmeticOperator::Add), ""))
        );
    }

    #[test]
    fn parse_subtract_without_whitespace() {
        assert_eq!(
            Operator::new("-"),
            Ok((Operator::Arithmetic(ArithmeticOperator::Sub), ""))
        );
    }

    #[test]
    fn parse_multiply_without_whitespace() {
        assert_eq!(
            Operator::new("*"),
            Ok((Operator::Arithmetic(ArithmeticOperator::Mul), ""))
        );
    }

    #[test]
    fn parse_divide_without_whitespace() {
        assert_eq!(
            Operator::new("/"),
            Ok((Operator::Arithmetic(ArithmeticOperator::Div), ""))
        );
    }

    #[test]
    fn parse_modulus_without_whitespace() {
        assert_eq!(
            Operator::new("%"),
            Ok((Operator::Arithmetic(ArithmeticOperator::Mod), ""))
        );
    }

    #[test]
    fn parse_equals_with_whitespace() {
        assert_eq!(
            Operator::new("  =  "),
            Ok((Operator::Assignment(AssignmentOperator::Equal), ""))
        );
    }

    #[test]
    fn parse_equals_without_whitespace() {
        assert_eq!(
            Operator::new("="),
            Ok((Operator::Assignment(AssignmentOperator::Equal), ""))
        );
    }

    #[test]
    fn parse_equals_equals_with_whitespace() {
        assert_eq!(
            Operator::new("  ==  "),
            Ok((Operator::Relational(RelationalOperator::EqualEqual), ""))
        );
    }

    #[test]
    fn parse_not_equals_with_whitespace() {
        assert_eq!(
            Operator::new("  !=  "),
            Ok((Operator::Relational(RelationalOperator::BangEquals), ""))
        );
    }

    #[test]
    fn parse_greater_with_whitespace() {
        assert_eq!(
            Operator::new("  >  "),
            Ok((Operator::Relational(RelationalOperator::Greater), ""))
        );
    }

    #[test]
    fn parse_greater_equal_with_whitespace() {
        assert_eq!(
            Operator::new("  >=  "),
            Ok((Operator::Relational(RelationalOperator::GreaterEqual), ""))
        );
    }

    #[test]
    fn parse_less_with_whitespace() {
        assert_eq!(
            Operator::new("  <  "),
            Ok((Operator::Relational(RelationalOperator::Less), ""))
        );
    }

    #[test]
    fn parse_less_equal_with_whitespace() {
        assert_eq!(
            Operator::new("  <=  "),
            Ok((Operator::Relational(RelationalOperator::LessEqual), ""))
        );
    }

    #[test]
    fn parse_equals_equals_without_whitespace() {
        assert_eq!(
            Operator::new("=="),
            Ok((Operator::Relational(RelationalOperator::EqualEqual), ""))
        );
    }

    #[test]
    fn parse_not_equals_without_whitespace() {
        assert_eq!(
            Operator::new("!="),
            Ok((Operator::Relational(RelationalOperator::BangEquals), ""))
        );
    }

    #[test]
    fn parse_greater_without_whitespace() {
        assert_eq!(
            Operator::new(">"),
            Ok((Operator::Relational(RelationalOperator::Greater), ""))
        );
    }

    #[test]
    fn parse_greater_equal_without_whitespace() {
        assert_eq!(
            Operator::new(">="),
            Ok((Operator::Relational(RelationalOperator::GreaterEqual), ""))
        );
    }

    #[test]
    fn parse_less_without_whitespace() {
        assert_eq!(
            Operator::new("<"),
            Ok((Operator::Relational(RelationalOperator::Less), ""))
        );
    }

    #[test]
    fn parse_less_equal_without_whitespace() {
        assert_eq!(
            Operator::new("<="),
            Ok((Operator::Relational(RelationalOperator::LessEqual), ""))
        );
    }

    #[test]
    fn parse_not_with_whitespace() {
        assert_eq!(
            Operator::new("  !  "),
            Ok((Operator::Logical(LogicalOperator::Not), ""))
        );
    }

    #[test]
    fn parse_and_with_whitespace() {
        assert_eq!(
            Operator::new("  &&  "),
            Ok((Operator::Logical(LogicalOperator::And), ""))
        );
    }

    #[test]
    fn parse_or_with_whitespace() {
        assert_eq!(
            Operator::new("  ||  "),
            Ok((Operator::Logical(LogicalOperator::Or), ""))
        );
    }

    #[test]
    fn parse_not_without_whitespace() {
        assert_eq!(
            Operator::new("!"),
            Ok((Operator::Logical(LogicalOperator::Not), ""))
        );
    }

    #[test]
    fn parse_and_without_whitespace() {
        assert_eq!(
            Operator::new("&&"),
            Ok((Operator::Logical(LogicalOperator::And), ""))
        );
    }

    #[test]
    fn parse_or_without_whitespace() {
        assert_eq!(
            Operator::new("||"),
            Ok((Operator::Logical(LogicalOperator::Or), ""))
        );
    }

    #[test]
    #[should_panic(expected = "Illegal Operator: @")]
    fn parse_illegal_operator() {
        match Operator::new("@") {
            Ok(res) => res,
            Err(e) => panic!("{}", e),
        };
    }
}
