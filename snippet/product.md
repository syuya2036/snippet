# 直積

```python
from itertools import product
A = [1, 2]
B = [4, 5]
C = [6, 7]

for p in product(A, B, C):
    print(p)
```

```python
# (1, 4, 6)
# (1, 4, 7)
# (1, 5, 6)
# (1, 5, 7)
# (2, 4, 6)
# (2, 4, 7)
# (2, 5, 6)
# (2, 5, 7)
```
