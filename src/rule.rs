use crate::{expr::Expr, Context};

#[derive(Debug, Clone)]
pub struct Rule {
    pub pattern: Expr,
    pub expr: Expr,
}

impl Rule {
    pub fn mr(&self, state: &Expr) -> Option<Expr> {
        matches(Context::new(), &self.pattern, state)
            .and_then(|context| self.expr.rewrite(&context))
    }
}

fn matches(context: Context, pattern: &Expr, state: &Expr) -> Option<Context> {
    match (pattern, state) {
        (Expr::Variable(v), state) => match context.get(v) {
            None => Some({
                let mut context = context;
                context.insert(v.clone(), state.clone());
                context
            }),
            Some(e) if e == state => Some(context),
            _ => None,
        },
        (Expr::List(l), Expr::List(ll)) => {
            let mut context = context;
            for (p, e) in l.iter().zip(ll.into_iter()) {
                context = matches(context, p, e)?;
            }
            Some(context)
        }
        (Expr::Symbol(s), Expr::Symbol(ss)) if s == ss => Some(context),
        _ => None,
    }
}
