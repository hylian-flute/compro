use proconio::input;

fn main() {
    input! {
        s: [char; 3],
        t: [char; 3],
    };

    let mut diff_count = 0_usize;
    for i in 0..3 {
        diff_count += if s[i] == t[i] { 0 } else { 1 };
    }

    let result = if diff_count == 0 || diff_count == 3 { "Yes" } else { "No" };
    println!("{}", result);
}
