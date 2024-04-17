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

