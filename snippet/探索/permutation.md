# 順列全探索

```python
>>> import itertools
>>> t=[i for i in range(4)]
>>> itertools.permutations(t)
<itertools.permutations object at 0x104dc0af0>
>>> list(itertools.permutations(t))
[(0, 1, 2, 3), (0, 1, 3, 2), (0, 2, 1, 3), (0, 2, 3, 1), (0, 3, 1, 2), (0, 3, 2, 1), (1, 0, 2, 3), (1, 0, 3, 2), (1, 2, 0, 3), (1, 2, 3, 0), (1, 3, 0, 2), (1, 3, 2, 0), (2, 0, 1, 3), (2, 0, 3, 1), (2, 1, 0, 3), (2, 1, 3, 0), (2, 3, 0, 1), (2, 3, 1, 0), (3, 0, 1, 2), (3, 0, 2, 1), (3, 1, 0, 2), (3, 1, 2, 0), (3, 2, 0, 1), (3, 2, 1, 0)]
>>> list(itertools.permutations(t,2))
[(3, 2), (3, 1), (3, 0), (2, 3), (2, 1), (2, 0), (1, 3), (1, 2), (1, 0), (0, 3), (0, 2), (0, 1)]
```
