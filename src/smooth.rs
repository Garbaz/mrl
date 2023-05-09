use crate::expr::Expr;


pub fn abstr(expr : &Expr) -> u64 {
    match expr {
        Expr::List(l) => 1 + l.iter().map(abstr).sum::<u64>(),
        Expr::Symbol(_) => 1,
        Expr::Variable(_) => 0,
    }
}