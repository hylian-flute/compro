use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let answer = if s[0] != s[1] && s[0] != s[2] {
        Some(s[0])
    } else if s[1] != s[0] && s[1] != s[2] {
        Some(s[1])
    } else if s[2] != s[0] && s[2] != s[1] {
        Some(s[2])
    } else {
        None
    };

    if let Some(c) = answer {
        println!("{}", c);
    } else {
        println!("-1");
    }
}
