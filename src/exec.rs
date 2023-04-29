use std::collections::BTreeMap;

use crate::{expr::Expr, rule::Rule};

type Context = BTreeMap<String, Expr>;

fn expr_m(context: Context, pattern: &Expr, expr: Expr) -> Option<Context> {
    match (pattern, expr) {
        (Expr::Variable(v), expr) => match context.get(v) {
            None => Some(Context::from([(v.clone(), expr)])),
            Some(e) if e == &expr => Some(context),
            _ => None,
        },
        (Expr::List(l), Expr::List(le)) => list_m(context, l, le),
        (Expr::Const(c), Expr::Const(ce)) if c == &ce => Some(Context::new()),
        _ => None,
    }
}

fn list_m(
    context: Context,
    pattern: &Vec<Expr>,
    expr: Vec<Expr>,
) -> Option<Context> {
    todo!()
}

pub fn rewrite(rules: Vec<Rule>, expr: Expr) -> Expr {
    todo!()
}
