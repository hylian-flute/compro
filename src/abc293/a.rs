use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut s = s;
    for i in 0..(s.len() / 2) {
        (s[2 * i], s[2 * i + 1]) = (s[2 * i + 1], s[2 * i]);
    }

    let result = s.iter().collect::<String>();
    println!("{}", &result);
}
