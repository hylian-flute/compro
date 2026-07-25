use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    };

    let result = [t, s].concat();
    println!("{}", &result);
}
