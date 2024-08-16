# 素数

## 素数判定

$n$が素数であるかを$O(\sqrt{N})$で判定する．

```cpp
bool is_prime(int n) {
    for(int i=2; i*i<=n; i++) {
        if (n % i == 0) return false;
    }
    return true;
}
```

## エラトステネスの篩

$N$以下の素数を$O(N\log\log N)$で列挙する．

```cpp
vector<bool> get_primes(int N) {
    vector<bool> primes(N+1);
    fill(primes.begin(), primes.end(), true);
    primes[0] = false; primes[1] = false;
    for(int i=2; i<N+1; i++) {
        if (!primes[i]) continue;
        for(int j=2*i; j<N+1; j+=i) primes[j] = false;
    }
    return primes;
}
```

## 素因数分解

```cpp
vector<pair<int, int>> prime_fact(int N) {
    vector<pair<int, int>> primes;
    int n = N;
    for(int i=2; i*i<=N; i++) {
        if (n % i != 0) continue;
        int cnt = 0;
        while(n % i == 0) {
            cnt++;
            n /= i;
        }
        primes.push_back(make_pair(i, cnt));

    }

    if (n != 1) primes.push_back(make_pair(n, 1));
    return primes;
}
```
