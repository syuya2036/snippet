# 強連結成分分解

## atcoder::scc_graph

```cpp
#include <bits/stdc++.h>
#include <atcoder/scc>
using namespace std;
#define vout(A) for (auto v : A) { cout << v << " "; } cout<<endl;
const long long INF = 1e18;

int main() {
    cin.tie(nullptr);
    ios::sync_with_stdio(false);
    int N, M;cin>>N>>M;
    atcoder::scc_graph g(N);
    for(int i=0; i<M; i++) {
        int A, B; cin>>A>>B;
        A--; B--;
        g.add_edge(A, B);
    }

    auto scc = g.scc();
    long long ans = 0;
    for(auto gr : scc) {
        long long n = gr.size();
        ans += n * (n-1) / 2;
    }
    cout<<ans<<endl;
}
```

## 自前

```cpp
#include <bits/stdc++.h>
#include <atcoder/scc>
using namespace std;
#define vout(A) for (auto v : A) { cout << v << " "; } cout<<endl;
const long long INF = 1e18;

int N, M;
vector<int> G[100005], rev[100005];
vector<int> vis(100005);
vector<int> back;
void dfs(int s) {
    vis[s] = true;
    for(int nxt : G[s]) {
        if (vis[nxt]) continue;
        dfs(nxt);
    }

    back.push_back(s);
}

vector<bool> rvis(100005);
vector<vector<int>> scc;
void rdfs(int s) {
    rvis[s] = true;
    scc.back().push_back(s);
    for(int prv : rev[s]) {
        if (rvis[prv]) continue;
        rdfs(prv);
    }
}


int main() {
    cin.tie(nullptr);
    ios::sync_with_stdio(false);
    cin>>N>>M;
    for(int i=0;i<M;i++) {
        int A, B; cin>>A>>B;
        A--;B--;
        G[A].push_back(B);
        rev[B].push_back(A);
    }

    for(int i=0; i<N; i++) {
        if(vis[i]) continue;
        dfs(i);
    }

    reverse(back.begin(), back.end());
    for(int s : back) {
        if(rvis[s]) continue;
        vector<int> g;
        scc.push_back(g);
        rdfs(s);
    }

    long long ans = 0;
    for(auto v : scc) {
        long long n = v.size();
        ans += n * (n - 1) / 2;
    }
    cout<<ans<<endl;
}

```

- [AC](https://atcoder.jp/contests/typical90/submissions/56318191)
