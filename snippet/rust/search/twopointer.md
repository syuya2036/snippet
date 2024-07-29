# しゃくとり法

## 二つ選んで差がK以下

```rust
fn main() {
    input! {
        N: usize,
        K: isize,
        mut A: [isize; N]
    }

    A.sort();
    let mut ans = 0;
    let mut q = VecDeque::new();
    q.push_back(A[0]);
    for r in 1..N {
        q.push_back(A[r]);
        while !q.is_empty() && A[r] - q.front().unwrap() > K {
            q.pop_front();
        }
        ans += q.len()
    }
    out!(ans - N + 1);
}
```

## 和がK以下になる区間の個数

```rust
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N]
    }

    let mut q = VecDeque::new();
    let mut s = 0;
    let mut ans = 0;
    for r in 0..N {
        q.push_back(A[r]);
        s += A[r];
        while !q.is_empty() && s > K {
            let rm = q.pop_front().unwrap();
            s -= rm;
        }
        ans += q.len();
    }

    println!("{}", ans);
}
```
