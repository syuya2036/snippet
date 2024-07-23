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
