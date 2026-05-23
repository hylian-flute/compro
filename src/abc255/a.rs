use proconio::{input, marker::Usize1};

fn main() {
    input! {
        r: Usize1,
        c: Usize1,
        a: [[i32; 2]; 2],
    };

    println!("{}", a[r][c]);
}
