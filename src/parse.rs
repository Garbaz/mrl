use peg::parser;

use crate::{expr::Expr, pattern::Pattern, rule::Rule};

parser! {
    pub grammar parser() for str {
        rule ws() = [' ' | '\n' | '\r' | '\t']
        rule _ = quiet!{ ws()* }
        rule __ = quiet!{ ws()+ }

        rule symbol() -> String
            = s:$([^ 'A'..='Z' | '=' | '(' | ')' | ' ' | '\n' | '\r' | '\t']
                  [^             '=' | '(' | ')' | ' ' | '\n' | '\r' | '\t']*) {s.to_string()}

        rule variable() -> String
            = v:$(['A'..='Z']['A'..='Z' | '_']*) {v.to_string()}

        rule elist() -> Expr
            = "(" _ l:(expr() ** __)  _ ")" {Expr::List(l)}

        rule esymbol() -> Expr
            = s:symbol() {Expr::Symbol(s)}

        rule evariable() -> Expr
            = v:variable() {Expr::Variable(v)}

        pub rule expr() -> Expr
            = elist() / esymbol() / evariable()

        rule plist() -> Pattern
            = "(" _ l:(pattern() ** __)  _ ")" {Pattern::List(l)}

        rule pvariable() -> Pattern
            = v:variable() {Pattern::Variable(v)}

        rule dotdot() -> Pattern
            = ".." {Pattern::DotDot}

        rule dotdotdot() -> Pattern
            = "..." {Pattern::DotDotDot}

        pub rule pattern() -> Pattern
            = plist() / pvariable() / dotdot() / dotdotdot()

        pub rule rule_() -> Rule
            = p:expr() _ "=" _ e:expr() {Rule{pattern: p, expr: e}}

        pub rule rules() -> Vec<Rule>
            = _ rs:(rule_() ** __) _ {rs}
    }
}
