pub mod utils;

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);
impl Number {
    pub fn new(s: &str) -> Self {
        Self(s.parse().unwrap())
    }
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod
}

impl Op {
    pub fn new(s: &str) -> Self {
        match s {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            "%" => Self::Mod,
            _ => panic!("Illegal Operator")

        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub first_operand: Number,
    pub second_operand: Number, 
    pub operator: Op
}

impl Expression {
    pub fn new(s: &str) -> Self {
        if s.len() != 3 {
            panic!("Illegal Expression");
        }
        
        let first_operand = Number::new(&s[..1]);
        let operator = Op::new(&s[1..2]);
        let second_operand = Number::new(&s[2..]);

        Self {first_operand, second_operand, operator}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), Number(123));
    }

    #[test]
    fn parse_add() {
        assert_eq!(Op::new("+"), Op::Add);
    }

    #[test]
    fn parse_subtract() {
        assert_eq!(Op::new("-"), Op::Sub);
    }

    #[test]
    fn parse_multiply() {
        assert_eq!(Op::new("*"), Op::Mul);
    }

    #[test]
    fn parse_divide() {
        assert_eq!(Op::new("/"), Op::Div);
    }

    #[test]
    fn parse_modulus() {
        assert_eq!(Op::new("%"), Op::Mod);
    }

    #[test]
    fn parse_expression() {
        assert_eq!(
            Expression::new("1+2"), 
            Expression {
                first_operand: Number(1),
                operator: Op::Add,
                second_operand: Number(2),
            });
    }
}
