// https://atcoder.jp/contests/abc375/tasks/abc375_d

use proconio::input;
use std::collections::HashMap;

#[derive(Clone, Copy)]
struct AboutChar {
    last_idx: usize,
    count: usize,
    last_score: usize,
}

fn main() {
    input! {
        s: String,
    };

    let mut char_to_about = HashMap::<char, AboutChar>::new();
    let result = s.chars().enumerate().fold(0_usize, |acc, (idx, value)| {
        let score = if let Some(about_char) = char_to_about.get(&value) {
            let new_score =
                about_char.last_score + about_char.count * (idx - about_char.last_idx) - 1;
            char_to_about.insert(
                value,
                AboutChar {
                    last_idx: idx,
                    count: about_char.count + 1,
                    last_score: new_score,
                },
            );
            new_score
        } else {
            char_to_about.insert(
                value,
                AboutChar {
                    last_idx: idx,
                    count: 1,
                    last_score: 0,
                },
            );
            0
        };
        acc + score
    });

    println!("{}", result);
}
