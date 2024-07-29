# ダイクストラ法

```cpp
#include <bits/stdc++.h>
using namespace std;
#define vout(A) for (auto v : A) { cout << v << " "; } cout<<endl;
const long long INF = 1e18;

int N, M, A, B, C;
vector<pair<int, int>> G[100005];
long long dist[100005];
int main() {
    cin.tie(nullptr);
    ios::sync_with_stdio(false);
    cin>>N>>M;
    while (M--) {
        cin>>A>>B>>C;
        A--;B--;
        G[A].push_back(make_pair(B, C));
        G[B].push_back(make_pair(A, C));
    }

    fill(dist, dist+100005, INF);
    priority_queue<pair<int, int>, vector<pair<int, int>>, greater<pair<int, int>>> q;
    q.push(make_pair(0, 0));
    dist[0] = 0;
    while (!q.empty()) {
        int v = q.top().second;q.pop();

        for (auto [s, c] : G[v]) {
            if (dist[s] <= dist[v] + c) continue;
            dist[s] = dist[v] + c;
            q.push(make_pair(dist[s], s));
        }
    }

    for (int i=0; i<N; i++) {
        if (dist[i] == INF) cout<< -1 << endl;
        else cout<<dist[i]<<endl;
    }
}
```
