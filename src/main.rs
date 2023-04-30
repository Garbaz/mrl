// use pest::Parser;
// use pest_derive::Parser;
use std::{fs, io, process::exit};

use mrl::{expr::Expr, parse3, rule::Rule};

// #[derive(Parser)]
// #[grammar = "expr.pest"]
// pub struct ExprParser;

fn rewrite(rules: &Vec<Rule>, state: &Expr) -> Option<Expr> {
    for r in rules {
        if let s @ Some(_) = r.mr(state) {
            return s;
        }
    }
    None
}

fn rewrites(rules: &Vec<Rule>, states: &Vec<Expr>) -> Option<Expr> {
    for s in states {
        if let s @ Some(_) = rewrite(rules, s) {
            return s;
        }
    }

    None
}

fn evaluate(rules: &Vec<Rule>, mut state: Expr) -> Expr {
    while let Some(s) = rewrite(rules, &state) {
        state = s;
    }
    state
}

fn main() {
    let source = fs::read_to_string("test.mrl").unwrap();

    let rules = parse3::parser::rules(&source).unwrap_or_else(|e| {
        println!("{}", e);
        exit(1)
    });

    // println!("{:?}", parse2::Parser::parse(parse2::Rule::rules, &file));

    // let (rem, rules) = parse::rules(&file).unwrap();

    // if !rem.is_empty() {
    //     println!("Unparsed rules:\n{}\n------", rem);
    // }

    for l in io::stdin().lines() {
        if let Ok(l) = l {
            match parse3::parser::expr(&l) {
                Ok(s) => {
                    println!("{}", evaluate(&rules, s))
                }
                Err(e) => println!("{}", e),
            }
            // match parse::expr(&l) {
            //     Ok((r, s)) if r.is_empty() => {
            //         println!("{}", evaluate(&rules, s))
            //     }
            //     Ok((r, _)) => println!("Excess characters: \"{}\"", r),
            //     Err(e) => println!("({}", e),
            // }
            // match ExprParser::parse(Rule::expr, &l) {
            //     Ok(p) => println!("{}", p),
            //     Err(e) => println!("{}", e),
            // }
        }
    }
}
