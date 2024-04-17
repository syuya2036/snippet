# ダイクストラ法

```python
N, M, r = map(int, input().split())

G = [[] for _ in range(N+1)]
for _ in range(M):
    u, v, w = map(int, input().split())
    G[u].append((v, w))

INF = 1e18
q = [(0, r)]
dist = [INF] * (N+1)
dist[r] = 0
while q:
    _, v = h.heappop(q)
    for e in G[v]:
        to, w = e[0], e[1]
        if dist[to] > dist[v] + w:
            h.heappush(q, (dist[v] + w, to))
            dist[to] = dist[v] + w

for d in dist[:-1]:
    if d == INF:
        print("INF")
    else:
        print(d)
```

