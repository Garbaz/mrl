use std::collections::BTreeMap;

use expr::Expr;

pub mod exec;
pub mod expr;
pub mod rule;
pub mod parse;
pub mod parse2;
pub mod parse3;

pub type Context = BTreeMap<String, Expr>;