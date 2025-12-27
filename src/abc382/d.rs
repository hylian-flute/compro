// https://atcoder.jp/contests/abc382/tasks/abc382_d

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    if 10 * (n - 1) + 1 > m {
        println!("0");
        return;
    }

    let mut number_vec = Vec::<usize>::new();
    for i in 0..n {
        number_vec.push(1 + 10 * i);
    }

    let mut result = Vec::<String>::new();

    loop {
        result.push(
            number_vec
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" "),
        );

        let mut changed_idx = Option::<usize>::None;
        for i in (0..n).rev() {
            if (i == n - 1 && number_vec[i] < m) || (i < n - 1 && number_vec[i] + 10 < number_vec[i + 1]) {
                changed_idx = Some(i);
                break;
            }
        }

        match changed_idx {
            Some(i) => {
                number_vec[i] += 1;
                for j in (i + 1)..n {
                    number_vec[j] = number_vec[j - 1] + 10;
                }
            },
            None => break,
        }
    }

    println!("{}", result.len());
    for row in result {
        println!("{}", row);
    }
}
