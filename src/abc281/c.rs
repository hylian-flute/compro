use proconio::input;

fn main() {
    input! {
        n: usize,
        t: i64,
        a: [i64; n],
    };

    let total_time = a.iter().sum::<i64>();
    let mut t = t % total_time;
    let mut idx = 0_usize;
    for i in 0..n {
        if t >= a[i] {
            t -= a[i];
        } else {
            idx = i;
            break;
        }
    }
    println!("{} {}", idx + 1, t);
}
