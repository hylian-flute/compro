use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [i32; n],

    };

    let mut max_sum = p[0..k].iter().sum::<i32>();
    let mut current_sum = max_sum;
    for i in k..n {
        current_sum += -p[i - k] + p[i];
        max_sum = max_sum.max(current_sum);
    }

    let result = (max_sum + k as i32) as f64 / 2.0;

    println!("{}", result);
}
