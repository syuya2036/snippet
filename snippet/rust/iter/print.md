# print
二次元vectorを表示する

```rust
fn printv2D<T: std::fmt::Debug>(v: &Vec<Vec<T>>) {
    for r in v {
        println!("{:?}", r);
    }
}
```

