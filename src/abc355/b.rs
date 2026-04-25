use std::{collections::HashSet, iter::FromIterator};
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i32; n],
        b: [i32; m],
    };
    let mut c = a.iter().chain(b.iter()).collect::<Vec<_>>();
    c.sort_unstable();
    let a_set = HashSet::<&i32>::from_iter(a.iter());

    let mut in_a = false;
    for c_i in c {
        if a_set.contains(&c_i) {
            if in_a {
                println!("Yes");
                return;
            }
            in_a = true;
        } else {
            in_a = false;
        }
    }
    println!("No");
}
