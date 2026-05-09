use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i32; n],
        b: [Usize1; m],
    };

    let mut result = 0_i32;
    for b_i in b {
        result += a[b_i];
    }

    println!("{}", &result);
}
