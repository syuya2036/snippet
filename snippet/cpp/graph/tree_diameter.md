# 木の直径

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
