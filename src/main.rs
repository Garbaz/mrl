use std::io;

use mrl::expr::expr_p;

fn main() {
    for l in io::stdin().lines() {
        if let Ok(l) = l {
            match expr_p(&l) {
                Ok((r, ex)) if r.is_empty() => println!("{}", ex),
                Ok((r, _)) => println!("Excess characters: `{}`", r),
                Err(er) => println!("({}", er),
            }
        }
    }
}
