use crate::{Context, expr::Expr};

pub enum Pattern {
    /// `(a b c)`
    List(Vec<Pattern>),

    /// `a`
    Symbol(String),

    /// `X`
    Variable(String),

    /// `..`
    DotDot,

    /// `...`
    DotDotDot,
}

impl Pattern {
    pub fn matches(&self, expr : &Expr) -> Context {
        use Pattern as P;
        use Expr as E;
        todo!()
    }
}