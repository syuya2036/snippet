# UnionFind木

```cpp
struct UnionFind {
    vector<int> parent;
    vector<int> rank;
    UnionFind(int N) {
        parent.resize(N, -1);
        rank.resize(N, -1);
    }

    int leader(int u) {
        if (parent[u] == -1) return u;
        else return parent[u] = leader(parent[u]);
    }

    bool same(int u, int v) {
        return leader(u) == leader(v);
    }

    void merge(int u, int v) {
        int pu = leader(u);
        int pv = leader(v);
        if (pu == pv) return;
        if (rank[pu] > rank[pv]) swap(pu, pv);
        if (rank[pu] == rank[pv]) rank[pv]++;
        parent[pu] = pv;
    }
};
```

- [AC](https://judge.yosupo.jp/submission/225264)
