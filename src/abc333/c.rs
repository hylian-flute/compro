use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut digits = vec![3_usize];
    let mut not_three_first_digit_i = 1_usize;
    for _ in 0..(n - 1) {
        if let Some(&not_three) = digits.get(not_three_first_digit_i) {
            digits = [
                vec![3],
                vec![not_three + 1; not_three_first_digit_i],
                digits[(not_three_first_digit_i + 1)..digits.len()].to_vec(),
            ]
            .concat();
            if not_three < 2 {
                not_three_first_digit_i = 1;
            } else {
                while not_three_first_digit_i < digits.len() && digits[not_three_first_digit_i] >= 3
                {
                    not_three_first_digit_i += 1;
                }
            }
        } else {
            digits = [vec![3], vec![1; not_three_first_digit_i]].concat();
            not_three_first_digit_i = 1;
        }
    }

    let answer = digits
        .iter()
        .rev()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join("");
    println!("{}", &answer);
}
