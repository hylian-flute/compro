// https://atcoder.jp/contests/abc353/tasks/abc353_d

// `a[i]` が `f(x,y)` の `x` に来る場合を考える。
// `a[i]` より後ろに `d` 桁の要素が `l_d` 個あるとすると合計値に `a[i] * 10^d * l_d` が加算される。
// `l_d` は制約より `0 <= d <= 9` のみ考えればよいため、合計値は `O(N)` で計算できる。
// `a[i]` が `f(x,y)` の `y` に来る場合では、合計値に `a[i] * i` が加算される。

use std::collections::HashMap;

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

/// 返り値 `vec` について `vec[i].get(exp) == c` と置いた場合、
/// `a[i + 1]`  以降の要素に `10^exp` 以上 `10^(exp + 1)` 未満の要素が `c` 個ある。
/// `0 <= exp <= 9` 。
fn generate_exp_cout_map_vec(n: usize, a: &Vec<i64>) -> Vec<HashMap<i64, usize>> {
    // `reverse` の呼び出しまでは順序が反転している
    let mut exp_count_map_vec = Vec::new();
    let mut acc_exp_count_map: HashMap<i64, usize> = HashMap::new();
    for exp in 0..=9 {
        acc_exp_count_map.insert(exp, 0);
    }
    exp_count_map_vec.push(acc_exp_count_map.clone());
    for i in (1..n).rev() {
        let exp = (a[i] as f64).log10() as i64;
        acc_exp_count_map.insert(exp, acc_exp_count_map.get(&exp).unwrap() + 1);
        exp_count_map_vec.push(acc_exp_count_map.clone());
    }

    exp_count_map_vec.reverse();
    exp_count_map_vec
}

fn calc_mod(v: i64) -> i64 {
    v % 998244353
}

fn mul_mod(v: &Vec<i64>) -> i64 {
    let mut ans = 1_i64;
    for v_i in v {
        ans = calc_mod(ans * calc_mod(*v_i));
    }
    ans
}

fn calc_effect_as_x(a_i: i64, exp_cout_map: &HashMap<i64, usize>) -> i64 {
    let mut effect = 0_i64;
    for exp in 0_i64..=9 {
        effect = calc_mod(
            effect
                + mul_mod(&vec![
                    a_i,
                    10_i64.pow(exp as u32 + 1),
                    *exp_cout_map.get(&exp).unwrap() as i64,
                ]),
        );
    }
    effect
}

fn calc_effect_as_y(i: usize, a_i: i64) -> i64 {
    mul_mod(&vec![a_i, i as i64])
}

fn slove(n: usize, a: &Vec<i64>) -> i64 {
    let exp_count_map_vec = generate_exp_cout_map_vec(n, a);
    let mut ans = 0_i64;
    for i in 0..n {
        let effect_as_x = calc_effect_as_x(a[i], &exp_count_map_vec[i]);
        let effect_as_y = calc_effect_as_y(i, a[i]);
        ans = calc_mod(ans + effect_as_x + effect_as_y);
    }
    ans
}

fn main() {
    let mut input = Input::new();
    let n: usize = input.get().parse().unwrap();
    let mut a: Vec<i64> = Vec::new();
    for _ in 0..n {
        a.push(input.get().parse().unwrap());
    }
    let output = slove(n, &a);
    println!("{}", &output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn about_generate_exp_cout_map_vec() {
        let n = 5_usize;
        let a: Vec<i64> = vec![17, 1, 100, 1000000000, 999];
        let exp_cout_map_vec = generate_exp_cout_map_vec(n, &a);
        for i in 0..(n - 1) {
            for exp in 0..=9 {
                let v = *exp_cout_map_vec[i].get(&exp).unwrap();
                if i == 0 && exp == 0 {
                    assert_eq!(v, 1_usize);
                } else if i == 0 && exp == 2 {
                    assert_eq!(v, 2_usize);
                } else if i == 0 && exp == 9 {
                    assert_eq!(v, 1_usize);
                } else if i == 1 && exp == 2 {
                    assert_eq!(v, 2_usize);
                } else if i == 1 && exp == 9 {
                    assert_eq!(v, 1_usize);
                } else if i == 2 && exp == 2 {
                    assert_eq!(v, 1_usize);
                } else if i == 2 && exp == 9 {
                    assert_eq!(v, 1_usize);
                } else if i == 3 && exp == 2 {
                    assert_eq!(v, 1_usize);
                } else {
                    assert_eq!(v, 0_usize);
                }
            }
        }
    }

    #[test]
    fn about_calc_effect_as_x() {
        let a_i = 17_i64;
        let exp_cout_map: HashMap<i64, usize> = HashMap::from([
            (0, 1),
            (1, 0),
            (2, 2),
            (3, 0),
            (4, 0),
            (5, 0),
            (6, 0),
            (7, 0),
            (8, 0),
            (9, 1),
        ]);
        let effect_as_x = calc_effect_as_x(a_i, &exp_cout_map);
        assert_eq!(effect_as_x, calc_mod(170 + 17000 * 2 + 170000000000));
    }

    #[test]
    fn about_slove_example() {
        let n = 5_usize;
        let a: Vec<i64> = vec![1001, 5, 1000000, 1000000000, 100000];
        let answer = slove(n, &a);
        assert_eq!(answer, 625549048);
    }

    #[test]
    fn about_slove_min() {
        let n = 2_usize;
        let a: Vec<i64> = vec![1, 1];
        let answer = slove(n, &a);
        assert_eq!(answer, 11);
    }

    #[test]
    fn about_slove_max() {
        let n: usize = 2 * 10_usize.pow(5);
        let mut a: Vec<i64> = Vec::new();
        for _ in 0..n {
            a.push(1000000000);
        }
        let answer = slove(n, &a);
        assert_eq!(answer, 871346439);
    }
}
