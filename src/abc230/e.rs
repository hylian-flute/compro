use proconio::input;

fn main() {
    input! {
        n: i64,
    };

    let mut result = 0_i64;
    let mut i = 1_i64;
    while i <= n {
        let v = n / i;
        let next_i = n / v + 1;
        result += v * (next_i - i);
        i = next_i;
    }

    println!("{}", result);
}
