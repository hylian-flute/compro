use proconio::input;

fn main() {
    input! {
        s: i32,
        t: i32,
    };

    let mut result = 0_usize;
    for a in 0..=s {
        for b in 0..=(s - a) {
            for c in 0..=(s - a - b) {
                if a * b * c <= t {
                    result += 1;
                }
            }
        }
    }

    println!("{}", result);
}
