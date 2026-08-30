use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
    };

    let mut sum_of_part = vec![a[0..m].iter().sum::<i64>()];
    for i in m..n {
        sum_of_part.push(sum_of_part.last().unwrap() - a[i - m] + a[i]);
    }

    let mut sum = 0_i64;
    for i in 0..m {
        sum += (i as i64 + 1) * a[i];
    }
    let mut answer = sum;
    for i in 0..(n - m) {
        sum = sum - sum_of_part[i] + m as i64 * a[i + m];
        if sum > answer {
            answer = sum;
        }
    }

    println!("{}", answer);
}
