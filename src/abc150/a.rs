use proconio::input;

fn main() {
    input! {
        k: usize,
        x: i32,
    };

    println!("{}", if 500 * (k as i32) >= x { "Yes" } else { "No" });
}
