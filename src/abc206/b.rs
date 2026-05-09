use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let d = (((2 * n) as f64 - 1.0 / 4.0).sqrt() - 1.0 / 2.0).ceil() as i32;
    println!("{}", d);
}
