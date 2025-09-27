// https://atcoder.jp/contests/abc373/tasks/abc373_d

use proconio::input;
use std::collections::HashMap;

const MAX: i128 = 1_000_000_000_000_000_000;

fn write(
    result: &mut Vec<i128>,
    written: &mut Vec<bool>,
    dependencies: &HashMap<usize, Vec<(usize, i128)>>,
    i: usize,
    value: i128,
) {
    let mut stack = vec![(i, value)];
    while let Some((k, value)) = stack.pop() {
        if written[k] {
            continue;
        }
        result[k] = value;
        if let Some(vec) = dependencies.get(&k) {
            for &(l, w) in vec {
                stack.push((l, value + w));
            }
        }
        written[k] = true;
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        vuw: [(usize, usize, i128); m],
    };

    let mut dependencies: HashMap<_, Vec<(usize, i128)>> = HashMap::new();
    for (v, u, w) in vuw {
        let v = v - 1;
        let u = u - 1;
        if let Some(vec) = dependencies.get_mut(&v) {
            vec.push((u, w));
        } else {
            dependencies.insert(v, vec![(u, w)]);
        }

        if let Some(vec) = dependencies.get_mut(&u) {
            vec.push((v, -w));
        } else {
            dependencies.insert(u, vec![(v, -w)]);
        }
    }

    let mut result = vec![0_i128; n];
    let mut written = vec![false; n];
    for i in 0..n {
        write(&mut result, &mut written, &dependencies, i, 0);
    }

    let max = result.iter().max().unwrap();
    let mut normalized_result = vec![0_i128; n];
    for i in 0..n {
        let n = result[i] - *max + MAX;
        normalized_result[i] = n;
    }
    let normalized_result: Vec<String> = normalized_result.iter().map(|n| n.to_string()).collect();
    println!("{}", normalized_result.join(" "));
}
