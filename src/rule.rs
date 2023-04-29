use nom::{
    bytes::complete::tag, combinator::map, sequence::separated_pair, IResult,
};

use crate::expr::{expr_p, Expr};

#[derive(Debug, Clone)]
pub struct Rule {
    pub lhs: Expr,
    pub rhs: Expr,
}

pub fn rule_p(input: &str) -> IResult<&str, Rule> {
    map(separated_pair(expr_p, tag("=>"), expr_p), |(lhs, rhs)| {
        Rule { lhs, rhs }
    })(input)
}
