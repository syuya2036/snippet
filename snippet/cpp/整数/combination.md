# 組み合わせの数

```cpp
#include <bits/stdc++.h>
using namespace std;
#define vout(A) for (auto v : A) { cout << v << " "; } cout<<endl;
const int INF = 1e8;

#include<atcoder/modint>
using mint = atcoder::modint1000000007;
mint fact(int n) {
    if (n == 1) return 1;
    return n*fact(n-1);
}

mint nPr(int n, int r) {
    mint ans = 1;
    for(int i=n; i>n-r; i--) ans *= i;
    return ans;
}

mint nCr(int n, int r) {
    mint fr = fact(r);
    mint p = nPr(n, r);
    mint ans = p*fr.inv();
    return ans;
}

int main() {
    cin.tie(nullptr);
    ios::sync_with_stdio(false);

    int n, r;
    cin>>n>>r;
    cout<<nCr(n, r).val()<<endl;
}
```

- [AC](https://atcoder.jp/contests/tessoku-book/submissions/56142638)
