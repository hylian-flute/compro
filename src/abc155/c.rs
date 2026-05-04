use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    };

    let mut count_from_str = HashMap::<String, usize>::new();
    let mut max_count = 0_usize;

    for s_i in s {
        let cur = *count_from_str.get(&s_i).unwrap_or(&0);
        count_from_str.insert(s_i, cur + 1);
        max_count = max_count.max(cur + 1);
    }

    let mut filtered = count_from_str
        .into_iter()
        .filter_map(|(str, count)| if count == max_count { Some(str) } else { None })
        .collect::<Vec<_>>();
    filtered.sort_unstable();

    println!("{}", filtered.join("\n"));
}
