# LowLink

## 橋の列挙

- 時間計算量: $O(V + E)$

```cpp
#include <bits/stdc++.h>
using namespace std;

struct LowLink {
    vector<int> ord, low;
    vector<vector<int>> G;
    vector<pair<int, int>> bridges;
    int N, k;

    LowLink(int N) : N(N), k(0), G(N), ord(N, -1), low(N, -1) {}

    void add(int u, int v) {
        G[u].emplace_back(v);
    }

    void build_ord(int v, int p) {
        ord[v] = low[v] = k++;
        for (auto nxt : G[v]) {
            if (nxt == p) continue; // 親ノードへのエッジは無視
            if (ord[nxt] == -1) { // 未探索のノード
                build_ord(nxt, v);
                low[v] = min(low[v], low[nxt]);
                if (ord[v] < low[nxt]) { // 橋の判定
                    if(v > nxt) bridges.emplace_back(nxt, v);
                    else bridges.emplace_back(v, nxt);
                }
            } else {
                low[v] = min(low[v], ord[nxt]);
            }
        }
    }

    void build() {
        for (int i = 0; i < N; ++i) {
            if (ord[i] == -1) {
                build_ord(i, -1);
            }
        }
    }
};

int main() {
    cin.tie(nullptr);
    ios::sync_with_stdio(false);

    int N, M;
    cin >> N >> M;
    LowLink ll(N);
    for (int i = 0; i < M; ++i) {
        int u, v;
        cin >> u >> v;
        ll.add(u, v);
        ll.add(v, u);
    }

    ll.build();
    vector<pair<int, int>> ans = ll.bridges;
    sort(ans.begin(), ans.end());

    for (const auto& bridge : ans) {
        cout << bridge.first << " " << bridge.second << endl;
    }

    return 0;
}

```

- [AC](https://onlinejudge.u-aizu.ac.jp/status/users/syuya2036/submissions/1/GRL_3_B/judge/9524281/C++17)

## 関節点の列挙

```cpp
#include <bits/stdc++.h>
using namespace std;

struct LowLink {
    vector<int> ord, low;
    vector<vector<int>> G;
    set<int> articulation_points;
    int N, k;

    LowLink(int N) : N(N), k(0), G(N), ord(N, -1), low(N, -1) {}

    void add(int u, int v) {
        G[u].emplace_back(v);
    }

    void build_ord(int v, int p) {
        ord[v] = low[v] = k++;
        int children = 0;
        for (auto nxt : G[v]) {
            if (nxt == p) continue; // 親ノードへのエッジは無視
            if (ord[nxt] == -1) { // 未探索のノード
                build_ord(nxt, v);
                low[v] = min(low[v], low[nxt]);
                children++;
                if (p != -1 && ord[v] <= low[nxt]) articulation_points.insert(v);
            } else {
                low[v] = min(low[v], ord[nxt]);
            }
        }

        if(p == -1 && children >= 2) articulation_points.insert(v);

    }

    void build() {
        for (int i = 0; i < N; ++i) {
            if (ord[i] == -1) {
                build_ord(i, -1);
            }
        }
    }
};

int main() {
    cin.tie(nullptr);
    ios::sync_with_stdio(false);

    int N, M;
    cin >> N >> M;
    LowLink ll(N);
    for (int i = 0; i < M; ++i) {
        int u, v;
        cin >> u >> v;
        ll.add(u, v);
        ll.add(v, u);
    }

    ll.build();
    for(auto ans : ll.articulation_points) cout<<ans<<endl;
}
```

- [AC](https://onlinejudge.u-aizu.ac.jp/status/users/syuya2036/submissions/1/GRL_3_A/judge/9524343/C++17)
