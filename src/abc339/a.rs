use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };

    let mut period_idx = 0_usize;
    for i in (0..s.len()).rev() {
        if s[i] == '.' {
            period_idx = i;
            break;
        }
    }
    let result = s[period_idx + 1..s.len()].iter().collect::<String>();
    println!("{}", &result);
}
