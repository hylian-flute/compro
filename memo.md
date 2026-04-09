## 型
### 整数型が安全に扱えるおおよその最大値
- `i32` : `2.1 * 10^9`
- `i64` : `9.2 * 10^18`
- `i128` : `1.7 * 10^38`
- `u32` : `4.2 * 10^9`
- `u64` (`usize`) : `1.8 * 10^19`
- `u128` : `3.4 * 10^38`

## 関数
### 整数の範囲を指定した二分探索
`x` を `left` 以上 `right` 以下の整数とする。
`pred` は引数が `x` 未満で常に `true` を返し、それ以上で常に `false` を返す関数。
`x` を返す。

```rs
fn partition_point_int(left: i32, right: i32, pred: impl Fn(i32) -> bool) -> i32 {
    let mut left = left;
    let mut right = right;

    if pred(right) {
        return right + 1;
    }

    while left < right {
        let mid = left + (right - left) / 2;
        if pred(mid) {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left
}
```

### ダイクストラ法

```rs
fn dijkstra(s: usize, v_len: usize, cost: &HashMap<(usize, usize), i32>) -> Vec<i32> {
    let mut d = vec![i32::MAX; v_len];
    let mut used = vec![false; v_len];
    d[s] = 0;

    loop {
        let mut v = Option::<usize>::None;

        for u in 0..v_len {
            if !used[u]
                && match v {
                    Some(some_v) => d[u] < d[some_v],
                    None => true,
                }
            {
                v = Some(u);
            }
        }

        if let Some(some_v) = v {
            used[some_v] = true;

            for u in 0..v_len {
                let key = if some_v < u { (some_v, u) } else { (u, some_v) };
                if let Some(c) = cost.get(&key) {
                    d[u] = i32::min(d[u], d[some_v] + c);
                }
            }
        } else {
            break;
        }
    }

    d
}
```