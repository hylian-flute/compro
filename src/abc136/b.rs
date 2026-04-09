use proconio::input;

fn main() {
    input! {
        n: i32,
    };

    let mut x = n;
    let mut result = 0usize;
    for exp in (0..=5).rev() {
        let base = 10i32.pow(exp);
        if x < base {
            continue;
        }
        if exp % 2 == 0 {
            result += (x - base + 1) as usize;
        }
        x = base - 1;
    }

    println!("{}", result);
}
