use proconio::input;

const MAX: usize = 200000;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut num_count = vec![0_usize; MAX + 1];
    for v in a {
        num_count[v] += 1;
    }

    let mut ge_count = vec![0_usize; MAX + 1];
    for v in 0..=MAX {
        let result: usize;
        if v == 0 {
            result = n;
        } else {
            result = ge_count[v - 1] - num_count[v - 1];
        }
        ge_count[v] = result;
    }

    let mut pattern_count = vec![vec![0_usize; MAX + 1]; 3];
    for order in (0..3_usize).rev() {
        for v in (0..=MAX).rev() {
            let result: usize;
            if order == 2 {
                result = ge_count[v];
            } else {
                if v == MAX {
                    result = 0;
                } else {
                    result = pattern_count[order][v + 1]
                        + (ge_count[v] - ge_count[v + 1]) * pattern_count[order + 1][v + 1];
                }
            }
            pattern_count[order][v] = result;
        }
    }

    println!("{}", pattern_count[0][0]);
}
