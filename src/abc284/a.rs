use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],

    };

    let result = s.into_iter().rev().collect::<Vec<_>>().join("\n");
    println!("{}", &result);
}
