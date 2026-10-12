use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let answer = (n * n.saturating_sub(1) + m * m.saturating_sub(1)) / 2;
    println!("{}", answer);
}
