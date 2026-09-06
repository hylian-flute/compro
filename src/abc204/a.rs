use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    };

    let answer = if x == y { x } else { 3 - x - y };
    println!("{}", answer);
}
