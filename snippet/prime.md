# 素数

## 素因数分解

```python
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
```

## 素数判定

```python
def is_prime(x):
    for i in range(2, x+1):
        if i*i > x:
            break
        if x % i == 0:
            return False

    return True
```

## オイラー関数

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

## エラトステネスの篩

```python
def get_primes(N):
    ans = [True] * (N+1)
    ans[0], ans[1] = False, False
    ans[2] = True
    for i in range(2, N+1):
        if not ans[i]:
            continue
        for j in range(2*i, N+1, i):
            ans[j] = False

    return ans
```

## 位数

```python
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

def primitive_root(p):
    ps = set(prime_fact(p-1))
    g = 2
    while True:
        for k in ps:
            if pow(g, (p-1)//k, p) == 1:
                g += 1
                break
        else:
            return g

print(primitive_root(7))  # 3
print(primitive_root(23))  # 5
print(primitive_root(1000000007))  # 5
print(primitive_root(998244353)) # 3
print(primitive_root(754974721)) # 11
```

