# pythonのsnippet集

## 二分探索
- binary

```python
def solve(x):
    return True

left = 0
right = 1e9
while right - left > 1:
    mid = (right + left) // 2
    if solve(mid):
        left = mid
    else:
        right = mid

print(left)
```

## bit全探索
- bit

```python
from itertools import product
