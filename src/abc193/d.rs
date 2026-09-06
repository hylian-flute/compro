use proconio::{input, marker::Chars};

fn char_to_usize(c: char) -> usize {
    match c {
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => panic!(),
    }
}

fn calc_score(count: &Vec<usize>, last: usize) -> usize {
    (1_usize..=9).fold(0_usize, |sum, i| {
        let c = count[i] + if i == last { 1 } else { 0 };
        sum + i * 10_usize.pow(c as u32)
    })
}

fn main() {
    input! {
        k: usize,
        s: Chars,
        t: Chars,
    };

    let mut count_of_s = vec![0_usize; 10];
    let mut count_of_t = vec![0_usize; 10];
    for i in 0..4 {
        count_of_s[char_to_usize(s[i])] += 1;
        count_of_t[char_to_usize(t[i])] += 1;
    }

    let mut answer = 0_f64;
    for n_s in 1_usize..=9 {
        let used_count_of_n_s = count_of_s[n_s] + count_of_t[n_s];
        if used_count_of_n_s >= k {
            continue;
        }
        let p_s = (k - used_count_of_n_s) as f64 / (9 * k - 8) as f64;
        let score_of_s = calc_score(&count_of_s, n_s);
        for n_t in 1_usize..=9 {
            let used_count_of_n_t =
                count_of_s[n_t] + count_of_t[n_t] + if n_t == n_s { 1 } else { 0 };
            if used_count_of_n_t >= k {
                continue;
            }
            let p_t = (k - used_count_of_n_t) as f64 / (9 * k - 9) as f64;
            let score_of_t = calc_score(&count_of_t, n_t);
            if score_of_s > score_of_t {
                answer += p_s * p_t;
            }
        }
    }

    println!("{}", answer);
}
