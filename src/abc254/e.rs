// https://atcoder.jp/contests/abc254/tasks/abc254_e

// パターンが2**19通りしかないから全探索で間に合う

use std::collections::{HashMap, HashSet};

struct Input {
    words: Vec<String>,
    p: usize,
}

impl Input {
    fn new() -> Self {
        Input {
            words: Vec::new(),
            p: 0,
        }
    }
    fn get(&mut self) -> &str {
        if self.words.len() <= self.p {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            for word in line.trim_end().split(' ') {
                self.words.push(String::from(word));
            }
        };
        self.p += 1;
        &self.words[self.p - 1]
    }
}

fn main() {
    let mut input = Input::new();
    let n: usize = input.get().parse().unwrap();
    let m: usize = input.get().parse().unwrap();
    let mut ab = Vec::new();
    for _i in 0..m {
        let a: usize = input.get().parse().unwrap();
        let b: usize = input.get().parse().unwrap();
        ab.push((a, b));
    }
    let q: usize = input.get().parse().unwrap();
    let mut xk = Vec::new();
    for _i in 0..q {
        let x: usize = input.get().parse().unwrap();
        let k: usize = input.get().parse().unwrap();
        xk.push((x, k));
    }

    let mut set_map = HashMap::new();
    for i in 1..(n + 1) {
        set_map.insert(i, HashSet::new());
    }
    for ab_i in ab {
        set_map.get_mut(&ab_i.0).unwrap().insert(ab_i.1);
        set_map.get_mut(&ab_i.1).unwrap().insert(ab_i.0);
    }

    let mut results: Vec<usize> = Vec::new();
    for xk_i in xk {
        let mut reachables = HashSet::new();
        let mut queue = vec![(xk_i.0, xk_i.1)];

        while queue.len() > 0 {
            let (vertex, count) = queue.pop().unwrap();
            reachables.insert(vertex);
            if count == 0 {
                continue;
            }
            let next_vertexes = set_map.get(&vertex).unwrap();
            for next_vertex in next_vertexes {
                queue.push((*next_vertex, count - 1));
            }
        }

        results.push(reachables.iter().sum());
    }

    for result in results {
        println!("{}", result);
    }
}
