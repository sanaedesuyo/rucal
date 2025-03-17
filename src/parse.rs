use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "math_parser.pest"]
struct MathParser;

use expr::Expr;
pub fn parse(expr: &str) -> Box<dyn Expr> {
    !unimplemented!()
}

pub mod expr {
    pub trait Expr {
        fn explain(&self) -> f64;
    }

}

#[cfg(test)]
mod tests {
    use pest::Parser;
    use crate::parse::{MathParser, Rule};

    #[test]
    pub fn test_parse() {
        let input = "2 * 2 * (3 - 1)";
        let pairs = MathParser::parse(Rule::expr, input).unwrap();

        for pair in pairs {
            println!("{:#?}", pair);
        }
    }
}