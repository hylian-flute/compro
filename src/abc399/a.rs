use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    };

    let mut d = 0_usize;
    for i in 0..n {
        if s[i] != t[i] {
            d += 1;
        }
    }
    println!("{}", &d);
}
