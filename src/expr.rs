use std::fmt;

use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::complete::{self, alpha1, space1},
    combinator::{map, verify},
    multi::separated_list0,
    sequence::delimited,
    IResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Symbol(String),
    Number(i64),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Symbol(s) => write!(f, "{}", s),
            Value::Number(n) => write!(f, "{}", n),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    List(Vec<Expr>),
    Const(Value),
    Variable(String),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::List(l) => {
                write!(f, "(")?;
                let mut l = l.iter();
                if let Some(e) = l.next() {
                    write!(f, "{}", e)?;
                }
                for e in l {
                    write!(f, " {}", e)?;
                }
                write!(f, ")")
            }
            Expr::Const(c) => write!(f, "{}", c),
            Expr::Variable(v) => write!(f, "{}", v),
        }
    }
}

pub fn expr_p(input: &str) -> IResult<&str, Expr> {
    alt((list_p, const_p, variable_p))(input)
}

fn list_p(input: &str) -> IResult<&str, Expr> {
    map(
        delimited(tag("("), separated_list0(space1, expr_p), tag(")")),
        Expr::List,
    )(input)
}

fn const_p(input: &str) -> IResult<&str, Expr> {
    map(alt((symbol_p, number_p)), Expr::Const)(input)
}

fn variable_p(input: &str) -> IResult<&str, Expr> {
    map(
        verify(alpha1, |s: &str| {
            s.chars().next().map_or(false, |c| c.is_lowercase())
        }),
        |s: &str| Expr::Variable(s.to_string()),
    )(input)
}

fn symbol_p(input: &str) -> IResult<&str, Value> {
    map(
        verify(alpha1, |s: &str| {
            s.chars().next().map_or(false, |c| c.is_uppercase())
        }),
        |s: &str| Value::Symbol(s.to_string()),
    )(input)
}

fn number_p(input: &str) -> IResult<&str, Value> {
    map(complete::i64, Value::Number)(input)
}
