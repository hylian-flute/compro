use proconio::{input, marker::Usize1};
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
        q: usize,
        query: [(Usize1, Usize1, i32); q],
    };

    let mut idxs_from_value = HashMap::<i32, Vec<usize>>::new();
    for i in 0..n {
        let value = a[i];
        if let Some(idxs) = idxs_from_value.get_mut(&value) {
            idxs.push(i);
        } else {
            idxs_from_value.insert(value, vec![i]);
        }
    }
    for idxs in idxs_from_value.values_mut() {
        idxs.sort_unstable();
    }

    for q_i in 0..q {
        let (l, r, x) = query[q_i];
        let q_answer = if let Some(idxs) = idxs_from_value.get(&x) {
            let l_pp = idxs.partition_point(|&idx| idx < l);
            let r_pp = idxs.partition_point(|&idx| idx <= r);
            r_pp - l_pp
        } else {
            0
        };
        println!("{}", q_answer);
    }
}
