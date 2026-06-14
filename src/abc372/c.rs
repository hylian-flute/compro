use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn is_abc(window: &[char]) -> bool {
    window[0] == 'A' && window[1] == 'B' && window[2] == 'C'
}

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        xc: [(Usize1, char); q],
    };

    let mut s = s;
    let mut count = 0_usize;
    for w in s.windows(3) {
        if is_abc(w) {
            count += 1;
        }
    }
    for (x, c) in xc {
        for w in s[x.saturating_sub(2)..(x + 3).min(n)].windows(3) {
            if is_abc(w) {
                count -= 1;
            }
        }
        s[x] = c;
        for w in s[x.saturating_sub(2)..(x + 3).min(n)].windows(3) {
            if is_abc(w) {
                count += 1;
            }
        }
        println!("{}", count);
    }
}
