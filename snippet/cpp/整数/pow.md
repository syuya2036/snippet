# 繰り返しニ乗法

## 二進展開

```cpp
#include <bits/stdc++.h>
using namespace std;
#define vout(A) for (auto v : A) { cout << v << " "; } cout<<endl;
const int INF = 1e8;

long long pow(long long x, long long a, long long mod) {
    long long ans = 1;
    while(a) {
        if (a&1 == 1) ans = ans * x % mod;
        a = a>>1;
        x = x*x % mod;
    }
    return ans;
}

int main() {
    cin.tie(nullptr);
    ios::sync_with_stdio(false);
    long long m, n;
    cin>>m>>n;
    cout<<pow(m, n, 1000000007)<<endl;
}
```

- [AC](https://onlinejudge.u-aizu.ac.jp/status/users/syuya2036/submissions/1/NTL_1_B/judge/9517126/C++17)

## 再帰

```cpp
long long pow(long long x, long long a, long long mod) {
    if (a == 0) return 1;
    if (a == 1) return x % mod;
    long long r = pow(x, a/2, mod);
    r = r*r % mod;
    if (a % 2 == 1) r = r * x % mod;
    return r;
}
```

- [AC](https://onlinejudge.u-aizu.ac.jp/status/users/syuya2036/submissions/1/NTL_1_B/judge/9517151/C++17)
