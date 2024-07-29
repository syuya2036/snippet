# bit全探索

## 部分和問題

```rust
#![allow(non_snake_case, unused_imports, unused_variables, dead_code, unused_macros)]
macro_rules! out { ($arg:expr) => { println!("{}", $arg); }; }
use proconio::{input, marker::{Chars, Usize1}};
use itertools::{Itertools, iproduct, sorted};
use itertools_num::ItertoolsNum as _;
use std::{collections::BinaryHeap, cmp::Reverse};
const INF: usize = 1<<60;

fn main() {
    input! {
        N: usize,
        S: usize,
        A: [usize; N]
    }

    for bit in 0..1<<N {
        let mut sum = 0;
        for i in 0..N {
            if bit & 1<<i > 0 {
                sum += A[i];
            }
        }

        if sum == S {
            out!("Yes");
            return;
        }
    }
    out!("No");
}
```
