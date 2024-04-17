# ユークリッドの互除法

## 最大公約数

```python 
def gcd(a, b):
    if b == 0:
        return a
    return gcd(b, a%b)
```
## 最小公倍数

```python
def lcm(a, b):
    return a*b//gcd(a,b)
```

## 拡張ユークリッドの互除法

ax + by = gcd(a, b)の整数解(x, y)を返す．

```python
def extgcd(a, b):
    if b == 0:
        return (1, 0)
    else:
        s, t = extgcd(b, a%b)
        return t, s - a//b * t
```

## 約数列挙

```python
def divisors(x):
    ans = []
    for i in range(1, x+1):
        if i*i > x:
            break

        if x % i != 0:
            continue
        ans.append(i)
        if x // i != i:
            ans.append(x // i)

    return sorted(ans)
```

