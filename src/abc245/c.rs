use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i32,
        a: [i32; n],
        b: [i32; n],
    };

    let mut memo = vec![[false; 2]; n];
    for i in 0..n {
        if i == 0 {
            memo[i][0] = true;
            memo[i][1] = true;
            continue;
        }
        for cur_a_or_b in 0..2_usize {
            let mut cur_result = false;
            let cur_value = if cur_a_or_b == 0 { a[i] } else { b[i] };
            for prev_a_or_b in 0..2_usize {
                if !memo[i - 1][prev_a_or_b] {
                    continue;
                }
                let prev_value = if prev_a_or_b == 0 { a[i - 1] } else { b[i - 1] };
                cur_result = cur_result || (cur_value - prev_value).abs() <= k;
            }
            memo[i][cur_a_or_b] = cur_result;
        }
    }

    let result = memo.last().unwrap().iter().any(|bl| *bl);
    println!("{}", if result { "Yes" } else { "No" });
}
