use std::fmt;

use crate::Context;


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    List(Vec<Expr>),
    Symbol(String),
    Variable(String),
}

impl Expr {
    pub fn rewrite(&self, context: &Context) -> Option<Expr> {
        match self {
            Expr::List(l) => Some(Expr::List(
                l.iter()
                    .map(|e| e.rewrite(context))
                    .collect::<Option<Vec<_>>>()?,
            )),
            e @ Expr::Symbol(_) => Some(e.clone()),
            Expr::Variable(v) => context.get(v).map(|e| e.clone()),
        }
    }

    pub fn is_value(&self) -> bool {
        match self {
            Expr::List(l) => l.iter().all(|e| e.is_value()),
            Expr::Symbol(_) => true,
            Expr::Variable(_) => false,
        }
    }
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
            Expr::Symbol(s) => write!(f, "{}", s),
            Expr::Variable(v) => write!(f, "{}", v),
        }
    }
}
