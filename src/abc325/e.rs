use std::collections::HashMap;
use proconio::input;

fn dijkstra(s: usize, v_len: usize, cost: &HashMap<(usize, usize), i64>) -> Vec<i64> {
    let mut d = vec![i64::MAX; v_len];
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
                    d[u] = i64::min(d[u], d[some_v] + c);
                }
            }
        } else {
            break;
        }
    }

    d
}

fn main() {
    input! {
        n: usize,
        a: i64,
        b: i64,
        c: i64,
        d: [[i64; n]; n],
    };

    let mut car_cost = HashMap::<(usize, usize), i64>::new();
    let mut train_cost = HashMap::<(usize, usize), i64>::new();
    for i in 0..n {
        for j in i..n {
            car_cost.insert((i, j), d[i][j] * a);
            train_cost.insert((i, j), d[i][j] * b + c);
        }
    }

    let car_d = dijkstra(0, n, &car_cost);
    let train_d = dijkstra(n - 1, n, &train_cost);

    let mut result = car_d[n - 1];
    for k in 0..n {
        let sum = car_d[k] + train_d[k];
        if sum < result {
            result = sum;
        }
    }

    println!("{}", result);
}
