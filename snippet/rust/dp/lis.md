# 最長増加部分列問題

```rust
fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    let mut dp = vec![1; N];
    let mut L = vec![INF; N];
    for i in 0..N {
        let pos = lower_bound(&L, &A[i]);
        dp[i] = pos;

        L[dp[i]] = A[i].min(L[dp[i]]);
    }

    out!(dp.iter().max().unwrap() + 1);
}

fn lower_bound<T: Ord>(A: &Vec<T>, v: &T) -> usize {
   return A.binary_search_by(|e| match e.cmp(v) {
        std::cmp::Ordering::Equal => std::cmp::Ordering::Greater,
        ord => ord,
    }).unwrap_err();
}
```
