# 文字列を埋める

```rust
fn fill(S: &String, c: char, N: usize) -> String {
    let l = N - S.len();
    let mut cn: String = std::iter::repeat(c).take(l).collect();
    cn.push_str(S);
    return cn
}
```

```rust
let x = "100";

// 右寄せ8桁
assert_eq!(format!("{x:>8} end"), "     100 end");
// 左寄せ8桁
assert_eq!(format!("{x:<8} end"), "100      end");
// 中央寄せ8桁
assert_eq!(format!("{x:^8} end"), "  100    end");
// ゼロ埋め右寄せ8桁
assert_eq!(format!("{x:0>8} end"), "00000100 end");
// *埋め左寄せ8桁
assert_eq!(format!("{x:*<8} end"), "100***** end");
```
