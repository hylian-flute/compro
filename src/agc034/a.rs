use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize,
        a: Usize1,
        b: Usize1,
        c: Usize1,
        d: Usize1,
        s: Chars,
    };

    if s[(a + 1)..c].windows(2).any(|w| w[0] == '#' && w[1] == '#') {
        println!("No");
        return;
    }

    if s[(b + 1)..d].windows(2).any(|w| w[0] == '#' && w[1] == '#') {
        println!("No");
        return;
    }

    if c > d {
        if !s[b.saturating_sub(1)..(d + 2).min(n)]
            .windows(3)
            .any(|w| w[0] == '.' && w[1] == '.' && w[2] == '.')
        {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
