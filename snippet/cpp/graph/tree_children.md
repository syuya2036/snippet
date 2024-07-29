# 木における各頂点の子の数

```cpp
#include <bits/stdc++.h>
using namespace std;
#define vout(A) for (auto v : A) { cout << v << " "; } cout<<endl;
const long long INF = 1e18;

int N, A;
int C[100005];
vector<int> G[100005];
int dfs(int v) {
    int children = G[v].size();
    for (auto nxt : G[v]) {
        children += dfs(nxt);
    }

    C[v] = children;
    return children;
}

int main() {
    cin.tie(nullptr);
    ios::sync_with_stdio(false);

    cin>>N;
    for (int i=1; i<N; i++){
        cin>>A;A--;
        G[A].push_back(i);
    }

    int ans = dfs(0);
    for (int i=0; i<N; i++) {
        cout<<C[i]<<" ";
    }

    cout<<endl;
}
```

- [AC](https://atcoder.jp/contests/tessoku-book/submissions/56097132)
