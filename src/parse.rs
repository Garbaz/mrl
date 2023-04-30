use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::complete::{alpha1, multispace1, line_ending},
    combinator::{map, verify},
    multi::separated_list0,
    sequence::{delimited, separated_pair},
    IResult,
};

use crate::{expr::Expr, rule::Rule};

pub fn expr(input: &str) -> IResult<&str, Expr> {
    alt((list, symbol, variable))(input)
}

fn list(input: &str) -> IResult<&str, Expr> {
    map(
        delimited(tag("("), separated_list0(multispace1, expr), tag(")")),
        Expr::List,
    )(input)
}

fn symbol(input: &str) -> IResult<&str, Expr> {
    map(
        verify(alpha1, |s: &str| {
            s.chars().next().map_or(false, |c| c.is_uppercase())
        }),
        |s: &str| Expr::Symbol(s.to_string()),
    )(input)
}

fn variable(input: &str) -> IResult<&str, Expr> {
    map(
        verify(alpha1, |s: &str| {
            s.chars().next().map_or(false, |c| c.is_lowercase())
        }),
        |s: &str| Expr::Variable(s.to_string()),
    )(input)
}

pub fn rule(input: &str) -> IResult<&str, Rule> {
    // println!("{}\n---------", input);
    map(separated_pair(expr, tag(" => "), expr), |(lhs, rhs)| Rule {
        pattern: lhs,
        expr: rhs,
    })(input)
}

pub fn rules(input: &str) -> IResult<&str, Vec<Rule>> {
    separated_list0(line_ending, rule)(input)
}

// pub fn pattern_p(input: &str) -> IResult<&str, Pattern> {
//     delimited(
//         multispace0,
//         alt((map(expr_p, Pattern::Expr), variable_p)),
//         multispace0,
//     )(input)
// }

// fn const_p(input: &str) -> IResult<&str, Expr> {
//     map(alt((symbol_p, number_p)), Expr::Const)(input)
// }

// fn symbol_p(input: &str) -> IResult<&str, Value> {
//     map(
//         verify(alpha1, |s: &str| {
//             s.chars().next().map_or(false, |c| c.is_uppercase())
//         }),
//         |s: &str| Value::Symbol(s.to_string()),
//     )(input)
// }

// fn number_p(input: &str) -> IResult<&str, Value> {
//     map(complete::i64, Value::Number)(input)
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum Value {
//     Symbol(String),
//     Number(i64),
// }

// impl fmt::Display for Value {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Value::Symbol(s) => write!(f, "{}", s),
//             Value::Number(n) => write!(f, "{}", n),
//         }
//     }
// }
