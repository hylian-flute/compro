use proconio::input;

fn main() {
    input! {
        s: String,
    };

    let answer = vec![s.clone(); 6 / s.len()].concat();
    println!("{}", &answer);
}
