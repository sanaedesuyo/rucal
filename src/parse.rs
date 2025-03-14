use std::str::FromStr;
use crate::parse::expr::{Expression, VarExpr};
use crate::parse::ParseError::{InvalidInput, UnexpectedToken};

pub enum ParseError {
    UnexpectedToken(String),
    InvalidInput(String),
}

pub mod expr {
    pub trait Expression {
        fn explain(&self) -> f64;
    }

    pub struct VarExpr {
        pub var: f64,
    }
    pub struct AddExpr {
        pub left: Box<dyn Expression>,
        pub right: Box<dyn Expression>,
    }
    pub struct SubExpr {
        pub left: Box<dyn Expression>,
        pub right: Box<dyn Expression>,
    }
    pub struct MulExpr {
        pub left: Box<dyn Expression>,
        pub right: Box<dyn Expression>,
    }
    pub struct DivExpr {
        pub left: Box<dyn Expression>,
        pub right: Box<dyn Expression>,
    }

    // impl
    impl Expression for VarExpr {
        fn explain(&self) -> f64 {
            self.var
        }
    }
    impl Expression for AddExpr {
        fn explain(&self) -> f64 {
            self.left.explain() + self.right.explain()
        }
    }
    impl Expression for SubExpr {
        fn explain(&self) -> f64 {
            self.left.explain() - self.right.explain()
        }
    }
    impl Expression for MulExpr {
        fn explain(&self) -> f64 {
            self.left.explain() * self.right.explain()
        }
    }
    impl Expression for DivExpr {
        fn explain(&self) -> f64 {
            let l = self.left.explain();
            let r = self.right.explain();

            if r == 0.0 {
                panic!("Divide by zero");
            }
            l / r
        }
    }
}

/// parse a string from math expression into `Box<dyn Expression>`
/// # Results
/// A box with Expression. `ParseError` if something wrong
pub fn parse(src: &str) -> Result<Box<dyn Expression>, ParseError> {
    Err(InvalidInput(String::from("输入错误")))
}

fn parse_unit(src: &str) -> Result<Box<dyn Expression>, ParseError> {
    let unit: Box<dyn Expression>;

    if src.starts_with('(') {
        let end_index = match src.find(')') {
            Some(index) => index,
            None => {
                return Err(UnexpectedToken(String::from("Parentheses not match")))
            }
        };

        unit = match parse_unit(&src[1..end_index]) {
            Ok(unit) => unit,
            Err(error) => return Err(error),
        };
    } else if src.as_bytes()[0].is_ascii_digit() {
        let mut end_index: u32 = 1;
        while "0123456789.".contains(src.as_bytes()[end_index as usize] as char) {
            end_index += 1;
        }

        unit = Box::new(VarExpr {
            var: match f64::from_str(&src[0..end_index as usize]) {
                Ok(value) => value,
                Err(_) => return Err(InvalidInput(String::from("Could not parse number"))),
            }
        });
    } else {
        return Err(UnexpectedToken(String::from("Not a number")));
    }

    Ok(unit)
}

fn parse_sign(src: &str) -> Result<Box<dyn Expression>, ParseError> {
    let mut left: Box<dyn Expression>;
    Err(UnexpectedToken(String::from("Illegal expression detected")))
}