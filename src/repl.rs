use std::{
    fmt,
    io::{self, Write},
};

use crate::{parse::parser, expr::Expr};

pub fn repl<T: fmt::Display, F: Fn(&str) -> anyhow::Result<T>>(
    prompt: &str,
    f: F,
) {
    let p = || {
        print!("{} ", prompt);
        io::stdout().flush().unwrap_or_default();
    };

    p();
    for l in io::stdin().lines() {
        if let Ok(l) = l {
            match f(&l) {
                Ok(r) => println!("{}", r),
                Err(e) => {
                    eprintln!("Error:");
                    eprintln!("{}", e)
                }
            }
        }
        p();
    }
}

pub fn prepl<T: fmt::Display, F: Fn(&Expr) -> T>(
    f_name: &str,
    f: F,
) {
    let mut prompt = f_name.to_string();
    prompt += "> ";

    repl(&prompt, |l| {
        let e = parser::expr(&l)?;
        Ok(f(&e))
    });
}

#[macro_export]
macro_rules! prepl {
    ($f:expr) => {
        ::mrl::repl::prepl(stringify!($f), $f)
    };
}