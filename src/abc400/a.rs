use proconio::input;

fn main() {
    input! {
        a: i32,
    };

    let answer = if 400 % a == 0 {
        400 / a
    } else {
        -1
    };

    println!("{}", answer);
}
