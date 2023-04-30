use pest::{iterators::{Pairs, Pair}, error::Error, Parser};
use pest_derive::Parser;

use crate::rule;

#[derive(Parser)]
#[grammar = "parse2.pest"]
struct P;

pub fn rule(input : &str) -> Result<rule::Rule, Error<Rule>> {
    let p = P::parse(Rule::rule, input)?.next().unwrap();

    fn r(pair : Pair<Rule>) -> rule::Rule {
        match pair.as_rule() {
            // Rule::rule => rule::Rule {},
            _ => todo!()
        }
    }

    Ok(r(p))
}

