# 木の直径

## 重みなし木の直径

```cpp
#include <bits/stdc++.h>
using namespace std;
#define vout(A) for (auto v : A) { cout << v << " "; } cout<<endl;
const int INF = 1e8;

int N, A, B;
vector<int> G[100005];

// (dist, v)
pair<int, int> max_dist_dfs(pair<int, int> v, int p) {
    pair<int, int> u = make_pair(v.first, v.second);
    for(int nxt : G[v.second]) {
        if (nxt == p) continue;
        u = max(u, max_dist_dfs(make_pair(v.first+1, nxt), v.second));
    }

    return u;
}

int main() {
    cin.tie(nullptr);
    ios::sync_with_stdio(false);

    cin>>N;
    for(int i=1;i<N;i++) {
        cin>>A>>B;A--;B--;
        G[A].push_back(B);
        G[B].push_back(A);
    }

    pair<int, int> p = max_dist_dfs(make_pair(1, 0), -1);
    pair<int, int> ans = max_dist_dfs(make_pair(1, p.second), -1);
    cout<<ans.first<<endl;
}


```

- [AC](https://atcoder.jp/contests/typical90/submissions/56142074)

## 重み付き木の直径(復元あり)

```cpp
#include <bits/stdc++.h>
using namespace std;
#define vout(A) for (auto v : A) { cout << v << " "; } cout<<endl;
const int INF = 1e8;

int N, A, B, C;
vector<pair<long long, int>> G[500005];

// (dist, v)
pair<long long, int> max_dist_dfs(pair<long long, int> v, int p) {
    pair<long long, int> u = v;
    for(auto nxt : G[v.second]) {
        if (nxt.second == p) continue;
        u = max(u, max_dist_dfs(make_pair(v.first + nxt.first, nxt.second), v.second));
    }
    return u;
}

vector<int> path;
void tree_path(int u, int v, int p) {
    if (u == v) {
        path.push_back(u);
        return;
    }
    if (path.size() != 0) return;

    for(auto [d, nxt] : G[u]) {
        if (nxt == p) continue;
        tree_path(nxt, v, u);
    }

    if (path.size() != 0) path.push_back(u);
}


int main() {
    cin.tie(nullptr);
    ios::sync_with_stdio(false);

    cin>>N;
    for(int i=1;i<N;i++) {
        cin>>A>>B>>C;
        G[A].push_back(make_pair(C, B));
        G[B].push_back(make_pair(C, A));
    }

    auto [mxu, u] = max_dist_dfs(make_pair(0, 0), -1);
    auto [mxv, v] = max_dist_dfs(make_pair(0, u), -1);
    tree_path(u, v, -1);

    reverse(path.begin(), path.end());
    cout<<mxv<<" "<<path.size()<<endl;
    vout(path);
}

```

- [AC](https://judge.yosupo.jp/submission/225278)
