// https://atcoder.jp/contests/abc374/tasks/abc374_e

use proconio::input;

fn partition_point_int(left: usize, right: usize, pred: impl Fn(usize) -> bool) -> usize {
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

fn cost_enough_w(w: usize, apbq: (usize, i64, usize, i64)) -> i64 {
    let (a, p, b, q) = apbq;

    let mut min_cost = 100 * 10_i64.pow(7);

    // apの機械をb個買うのはbqの機械をa個買うのと生産量が a*b で同じ。
    // つまりapがb個以上でbqもa個以上という解にはならない。
    // （それよりどちらか安い方に置き換える方が明らかに良い）
    // よってapがb個未満の全パターンとbqがa個未満の全パターンを探索する。

    for ap_count in 0..b {
        let ap_w = a * ap_count;
        let remain_w = w.saturating_sub(ap_w);
        let bq_count = (remain_w + b - 1) / b;
        let cost = p * ap_count as i64 + q * bq_count as i64;

        if cost < min_cost {
            min_cost = cost;
        }
    }

    for bq_count in 0..a {
        let bq_w = b * bq_count;
        let remain_w = w.saturating_sub(bq_w);
        let ap_count = (remain_w + a - 1) / a;
        let cost = p * ap_count as i64 + q * bq_count as i64;

        if cost < min_cost {
            min_cost = cost;
        }
    }

    min_cost
}

fn main() {
    input! {
        n: usize,
        x: i64,
        apbq: [(usize, i64, usize, i64); n],
    };

    let result = partition_point_int(0, 100 * 10_usize.pow(7), |w| {
        let mut remain_x = x;
        for i in 0..n {
            let cost = cost_enough_w(w, apbq[i]);
            if cost > remain_x {
                return false;
            } else {
                remain_x -= cost;
            }
        }

        true
    }) - 1;

    println!("{}", result);
}
