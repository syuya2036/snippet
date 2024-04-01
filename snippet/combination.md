# 組み合わせ

```python
def fact(n, mod):
    ans = 1
    for i in range(2, n+1):
        ans *= i
        ans %= mod
    return ans

def nCr(n, r, mod):
    fr = fact(r, mod)
    p = nPr(n, r, mod)

    return p * pow(fr, -1, mod) % mod

def nPr(n, r, mod):
    ans = 1
    for i in range(n, n-r, -1):
        ans *= i
        ans %= mod
    return ans
```
