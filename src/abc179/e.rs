use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        x: i64,
        m: i64,
    };

    let mut memo = HashMap::<i64, (usize, i64)>::new();

    let mut i = 0_usize;
    let mut a_i = x;
    let mut sum = 0_i64;
    while i < n {
        if let Some((j, sum_of_j)) = memo.get(&a_i) {
            let sum_diff = sum - sum_of_j;
            let idx_diff = i - j;
            let count = (n - i - 1) / idx_diff;
            sum += sum_diff * (count as i64);
            i += idx_diff * count;
        }

        memo.insert(a_i, (i, sum));

        sum += a_i;
        i += 1;
        a_i = a_i.pow(2) % m;
    }

    println!("{}", sum);
}
