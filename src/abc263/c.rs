use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let mut queue = VecDeque::<Vec<usize>>::from([vec![]]);
    while let Some(partial) = queue.pop_front() {
        if partial.len() == n {
            let s = partial
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            println!("{}", &s);
            continue;
        }
        let last_num = partial.last().unwrap_or(&0);
        for i in (*last_num + 1)..=m {
            queue.push_back([partial.clone(), vec![i]].concat());
        }
    }
}
