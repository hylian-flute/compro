use proconio::input;

fn main() {
    input! {
        x: i32,
    };

    // B が負のパターン
    // 64^5 は 10^5 より大きい
    // A も B も負の解があるなら必ず A も B も正の解があるので考慮しなくてよい
    for minus_b in 0..=63_i32 {
        for a in 0..=63_i32 {
            if a.pow(5) + minus_b.pow(5) == x {
                println!("{} {}", a, -minus_b);
                return;
            }
        }
    }

    // B が正のパターン
    // 120^5 - 119^5 は 10^9 より大きい
    for a in 0..=119_i32 {
        for b in 0..a {
            if a.pow(5) - b.pow(5) == x {
                println!("{} {}", a, b);
                return;
            }
        }
    }
}
