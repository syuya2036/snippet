#![allow(non_snake_case, unused_imports, unused_variables, dead_code)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: usize,
        X: usize,
        A: [usize; N]
    }

    println!("{}", if A.contains(&X) {"Yes"} else {"No"});
}
