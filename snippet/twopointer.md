# しゃくとり法

数列Aにおいて，総和がx以下になるような範囲の個数．

```python
from collections import deque
N, Q = map(int, input().split())
A = list(map(int, input().split()))
X = list(map(int, input().split()))

for x in X:
    q = deque()
    s = 0
    ans = 0
    for i in range(N):
        q.append(A[i])
        s += A[i]
        while q and s > x:
            rm = q.popleft()
            s -= rm
        # ここでqは条件を満たす状態となり，かつ網羅する．
        ans += len(q)

    print(ans)
```

https://qiita.com/keroru/items/6e0a22e8c9bf2a24dc68
