# トポロジカルソート

```cpp
#include <bits/stdc++.h>
using namespace std;
#define vout(A) for (auto v : A) { cout << v << " "; } cout<<endl;
const long long INF = 1e18;

int N, M;
vector<int> G[10005];
bool vis[10005];
bool in[10005];
vector<int> ans;
void dfs(int v) {
    vis[v] = true;
    for(int nxt : G[v]) {
        if (vis[nxt]) continue;
        dfs(nxt);
    }

    ans.push_back(v);
}

int main() {
    cin.tie(nullptr);
    ios::sync_with_stdio(false);

    cin>>N>>M;
    for(int i=0;i<M;i++) {
        int u, v; cin>>u>>v;
        G[u].push_back(v);
        in[v] = true;
    }

    for(int i=0;i<N;i++) {
        if (in[i]) continue;
        if (vis[i]) continue;
        dfs(i);
    }

    for(int i=N-1;i>=0;i--) cout<<ans[i]<<endl;
}
```
