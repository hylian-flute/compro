use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        t: usize,
    };

    let answer = t * ((n + x - 1) / x);
    println!("{}", answer);
}
