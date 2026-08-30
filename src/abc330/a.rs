use proconio::input;

fn main() {
    input! {
        n: usize,
        l: i32,
        a: [i32; n],
    };

    let answer = a
        .iter()
        .fold(0, |acc, &cur| acc + if cur >= l { 1 } else { 0 });
    println!("{}", answer);
}
