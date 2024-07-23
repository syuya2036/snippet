# 累乗

## 繰り返し2乗法

```python
def power(a, b, mod):
    if b == 0:
        return 1
    if b == 1:
        return a % mod
    r = b % 2
    q = b // 2
    return power(a, q, mod) ** 2 * power(a, r, mod) % mod

```
