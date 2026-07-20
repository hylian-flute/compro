use proconio::input;

fn main() {
    input! {
        t: i32,
    };

    let f = |x: i32| x.pow(2) + 2 * x + 3;
    let result = f(f(f(t) + t) + f(f(t)));

    println!("{}", result);
}
