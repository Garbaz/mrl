use peg::parser;

use crate::{expr::Expr, rule::Rule};

parser! {
    pub grammar parser() for str {
        rule ws() = [' ' | '\n' | '\r' | '\t']
        rule _ = quiet!{ ws()* }
        rule __ = quiet!{ ws()+ }

        rule variable() -> Expr
            = "." v:$(['a'..='z' | '_']+) {Expr::Variable(v.to_string())}

        rule symbol() -> Expr
            = s:$([^ '(' | ')' | '_' | '.' | ' ' | '\n' | '\r' | '\t']+) {Expr::Symbol(s.to_string())}

        rule list() -> Expr
            = "(" _ l:(expr() ** __)  _ ")" {Expr::List(l)}

        pub rule expr() -> Expr
            = list() / variable() / symbol()

        pub rule rule_() -> Rule
            = p:expr() _ "=>" _ e:expr() {Rule{pattern: p, expr: e}}

        pub rule rules() -> Vec<Rule>
            = rule_() ** __
    }
}