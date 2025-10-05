## 整数の範囲を指定した二分探索
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
