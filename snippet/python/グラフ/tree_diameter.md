# 木の直径

```python
import sys
# sys.setrecursionlimit(10**9)
def input():
    return sys.stdin.readline().rstrip()

from collections import deque


def dfs(v, n):
    vis = [-1] * (n+1)
    q = deque()
    q.append(v)
    vis[v] = 0
    while q:
        v = q.pop()
        for s in G[v]:
            if vis[s] != -1:
                continue
            vis[s] = vis[v] + 1
            q.append(s)
    
    mx = 0
    ans = -1
    for i, v in enumerate(vis):
        if mx < v:
            mx = v
            ans = i

    return mx, ans

N = int(input())
G = [[] for _ in range(N+1)]
for _ in range(1, N):
    u, v = map(int, input().split())
    G[u].append(v)
    G[v].append(u)


_, v = dfs(1, N)
d, _ = dfs(v, N)
print(d + 1)
```
