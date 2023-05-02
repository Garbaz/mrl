use std::{
    fs,
    io::{self, Write},
    iter,
    process::exit, env,
};

use mrl::{expr::Expr, parse, rule::Rule};

fn rewrites(rules: &Vec<Rule>, mut states: Vec<&mut Expr>) -> Option<Rule> {
    for s in states.iter_mut() {
        for r in rules {
            if r.mr(s).is_some() {
                return Some(r.clone());
            }
        }
    }
    let states = states
        .iter_mut()
        .flat_map(|e| -> Box<dyn Iterator<Item = &mut Expr>> {
            if let Expr::List(l) = e {
                Box::new(l.iter_mut())
            } else {
                Box::new(iter::empty())
            }
        })
        .collect::<Vec<_>>();
    if !states.is_empty() {
        if let Some(r) = rewrites(rules, states) {
            return Some(r);
        }
    }
    None
}

fn rewrite(rules: &Vec<Rule>, state: &mut Expr) -> Option<Rule> {
    rewrites(rules, vec![state])
}

fn evaluate(debug: bool, rules: &Vec<Rule>, state: &mut Expr) {
    if debug {
        println!("{}", state);
    }
    while let Some(r) = rewrite(rules, state) {
        if debug {
            println!("{}", r);
            println!("{}", state);
        }
    }
    if !debug {
        println!("{}", state);
    }
}

fn prompt() {
    print!("mrl> ");
    io::stdout().flush().unwrap_or_default();
}

fn usage() {
    println!("Usage:");
    println!("  mrl RULES_FILE");
}

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let filename = args.get(1).unwrap_or_else(|| {
        usage();
        exit(1)
    });

    let source = fs::read_to_string(filename).unwrap();

    let rules = parse::parser::rules(&source).unwrap_or_else(|e| {
        println!("Could not parse the rules:");
        println!("{}", e);
        exit(2)
    });

    let mut debug = false;

    prompt();
    for l in io::stdin().lines() {
        if let Ok(l) = l {
            if l == "=debug" {
                debug = !debug;
                println!("Toggled debug to {}", debug);
            } else {
                match parse::parser::expr(&l) {
                    Ok(mut s) => evaluate(debug, &rules, &mut s),
                    Err(e) => {
                        println!("Could not parse the expression:");
                        println!("{}", e)
                    }
                }
            }
        }
        prompt();
    }
}
