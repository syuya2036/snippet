# 木の重心

```cpp
int N;
vector<int> G[200005];
int sz[200005];
int center;
int centroid(int v, int p) {
    sz[v] = 1;
    int mxc = 0;
    for(int nxt : G[v]) {
        if (nxt == p) continue;
        sz[v] += centroid(nxt, v);
        mxc = max(mxc, sz[nxt]);
    }

    mxc = max(mxc, N-sz[v]);
    if (mxc*2<=N) center = v;
    return sz[v];
}
```
