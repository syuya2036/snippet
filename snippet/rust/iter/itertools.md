# itertools

- [競プロで使えそうな Rust の itertools のマクロとメソッドまとめ](https://qiita.com/okaponta_/items/8d6032b8f2b664c24ad1)
- [docs](https://docs.rs/itertools/latest/itertools/trait.Itertools.html)

## iproduct

n 重 for 文を一つの for 文で書ける。

```rust
#![allow(non_snake_case, unused_imports, unused_variables, dead_code)]
use proconio::{input, marker::Chars};
use itertools::iproduct;

fn main() {
    input! {
        N: usize,
        K: usize,
        P: [usize; N],
        Q: [usize; N],
    }

    for (i, j) in iproduct!(0..N, 0..N) {
        if P[i] + Q[j] == K {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
```

## combinations

配列 A から 3 つ選んで和が 1000 になる組み合わせがあるかどうかを判定する。

```rust
#![allow(non_snake_case, unused_imports, unused_variables, dead_code)]
use proconio::{input, marker::Chars};
use itertools::Itertools;

fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

   for l in (0..N).combinations(3) {
       if  A[l[0]] + A[l[1]] + A[l[2]] == 1000 {
           println!("Yes");
           return;
       }
   }

    println!("No");
}
```

## 累積和

```rust
#![allow(non_snake_case, unused_imports, unused_variables, dead_code)]
use proconio::{input, marker::Chars};
use itertools::{Itertools, iproduct};
use itertools_num::ItertoolsNum as _;

fn main() {
    input! {
        N: usize,
        Q: usize,
        mut A: [usize; N]
    }

    A.insert(0, 0);
    let X = A.iter().cumsum().collect::<Vec<usize>>();
    for _ in 0..Q {
      input! {
        l: usize,
        r: usize
      }
      println!("{}", X[r] - X[l-1]);
    }
}
```
