# bit

```rust
fn to_base_n(mut num: usize, base: usize) -> String {
    // 2 以上の base かどうかをチェック
    assert!(base >= 2, "Base must be at least 2");

    // 0 の場合は "0" を返す
    if num == 0 {
        return "0".to_string();
    }

    let mut result = Vec::new();
    while num > 0 {
        let remainder = num % base;
        result.push(std::char::from_digit(remainder as u32, base as u32).unwrap());
        num /= base;
    }

    result.iter().rev().collect()
}
```

```rust
fn main() {
    input! {
        N: String,
    }

    let ans = usize::from_str_radix(&N, 2).unwrap();
    println!("{}", ans);
}
```
