use std::collections::BTreeMap;
use proconio::input;

fn main() {
    input! {
        n: usize,
        w: i64,
        stp: [(usize, usize, i64); n],
    };

    let mut events = BTreeMap::<usize, i64>::new();
    for (s, t, p) in stp {
        let cur_of_s = events.get(&s).unwrap_or(&0);
        events.insert(s, cur_of_s + p);

        let cur_of_t = events.get(&t).unwrap_or(&0);
        events.insert(t, cur_of_t - p);
    }

    let mut using = 0_i64;
    for diff in events.values() {
        using += diff;
        if using > w {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
