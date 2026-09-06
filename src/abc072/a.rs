use proconio::input;

fn main() {
    input! {
        x: i32,
        t: i32,
    };

    let answer = (x - t).max(0);
    println!("{}", answer);
}
