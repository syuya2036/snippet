# ビット全探索

```python
from itertools import product

for p in product([0, 1], repeat=3):
	print(p)
```

```python
# (0, 0, 0)
# (0, 0, 1)
# (0, 1, 0)
# (0, 1, 1)
# (1, 0, 0)
# (1, 0, 1)
# (1, 1, 0)
# (1, 1, 1)
```

## 例

### 部分和問題

```python
from itertools import product
N, K = map(int, input().split())
A = list(map(int, input().split()))

ok = False
for p in product([0, 1], repeat=N):
	s = 0
	for bit in p:
		if bit == 1:
			s += A[bit]

	if s == K:
		ok = True
		break

print("Yes" if ok else "No")
```
