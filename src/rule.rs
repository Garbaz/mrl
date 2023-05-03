use core::fmt;

use crate::{expr::Expr, Context};

#[derive(Debug, Clone)]
pub struct Rule {
    pub pattern: Expr,
    pub expr: Expr,
}

impl Rule {
    pub fn mr(&self, state: &mut Expr) -> Option<()> {
        let context = matches(Context::new(), &self.pattern, state)?;
        *state = self.expr.rewrite(&context)?;
        Some(())
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
            if l.len() == ll.len() {
                let mut context = context;
                for (p, e) in l.iter().zip(ll.iter()) {
                    context = matches(context, p, e)?;
                }
                Some(context)
            } else {
                None
            }
        }
        (Expr::Symbol(s), Expr::Symbol(ss)) if s == ss => Some(context),
        _ => None,
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.pattern, self.expr)
    }
}
