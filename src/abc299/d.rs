use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut left = 0;
    let mut right = n;

    while left < right {
        let mid = left + (right - left) / 2;
        println!("? {}", mid + 1);
        input! {
            mid_v: i32,
        };
        if mid_v == 0 {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    println!("! {}", left);
}
