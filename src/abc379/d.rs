// https://atcoder.jp/contests/abc379/tasks/abc379_d

use proconio::input;

fn main() {
    input! {
        q: usize,
    };

    let mut total_t = 0usize;
    let mut planted_days = Vec::<usize>::new();
    let mut harvested_index = 0usize;
    let mut result = Vec::<usize>::new();
    for _ in 0..q {
        input! {
            command: usize,
        };

        match command {
            1 => {
                planted_days.push(total_t);
            }
            2 => {
                input! {
                    t: usize,
                };
                total_t += t;
            }
            _ => {
                input! {
                    h: usize,
                };
                let new_harvested_index = planted_days.partition_point(|v| total_t - *v >= h);
                if new_harvested_index > harvested_index {
                    result.push(new_harvested_index - harvested_index);
                    harvested_index = new_harvested_index;
                } else {
                    result.push(0);
                };
            }
        };
    }

    for n in result.iter() {
        println!("{}", n);
    }
}
