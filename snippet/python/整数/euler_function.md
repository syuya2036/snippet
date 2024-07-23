# オイラー関数

```python
from collections import Counter

def prime_fact(x):
    ans = []
    tmp = x
    for i in range(2, x+1):
        if i*i > tmp:
            break
        while x%i == 0:
            ans.append(i)
            x //= i

    if x != 1:
        ans.append(x)
    return ans

def euler(x):
    primes = Counter(prime_fact(x))
    ans = 1
    for p, e in primes.items():
        ans *= (p ** e - p ** (e-1))
    return ans
```
