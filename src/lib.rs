use std::collections::BTreeMap;

use expr::Expr;

pub mod exec;
pub mod expr;
pub mod rule;
pub mod parse;
pub mod pattern;
pub mod smooth;
pub mod repl;

pub type Context = BTreeMap<String, Expr>;