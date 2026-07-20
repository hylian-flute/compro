use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize
    };

    let mut a = Vec::<HashSet<usize>>::new();
    for _ in 0..m {
        input! {
            c: usize,
            a_i: [usize; c],
        };
        a.push(a_i.into_iter().collect());
    }

    let mut result = 0_usize;
    for i in 0..2_usize.pow(m as u32) {
        let mut b_all = true;
        for required_num in 1..=n {
            let mut b_any = false;
            for a_idx in 0..m {
                b_any = b_any || ((i >> a_idx) % 2 == 1 && a[a_idx].contains(&required_num));
            }
            b_all = b_all && b_any;
        }
        if b_all {
            result += 1;
        }
    }

    println!("{}", result);
}
