# ダイクストラ法

```rust

struct Dijkstra {
    // 0-indexed
    dist: Vec<usize>,
    from: Vec<usize>
}

impl Dijkstra {
    fn new(N: usize, G: Vec<Vec<(usize, usize)>>, start: usize) -> Self {
        let mut dist = vec![INF; N];
        let mut from = vec![INF; N];
        let mut hq = BinaryHeap::new();
        hq.push((Reverse(0), start));
        dist[start] = 0;
        while !hq.is_empty() {
            let (_, p) = hq.pop().unwrap();
            for &(c, v) in &G[p] {
                if dist[v] <= dist[p] + c {
                    continue;
                }
                dist[v] = dist[p] + c;
                from[v] = p;
                hq.push((Reverse(dist[v]), v));
            }
        }
        Self { from, dist }
    }
    fn path(&self, to: usize) -> Vec<usize> {
        let mut path = vec![to];
        let mut now = to;
        while self.from[now] != INF {
            let prev = self.from[now];
            path.push(prev);
            now = prev;
        }
        path.reverse();path
    }

}

```
