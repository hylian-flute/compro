use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        sc: [(Usize1, char); m],
    };

    let mut result = vec![Option::<char>::None; n];
    let mut not_exist = false;

    for (s, c) in sc.iter() {
        let duplicate = if let Some(v) = result[*s] {
            v != *c
        } else {
            false
        };
        let first_zero = n > 1 && *s == 0 && *c == '0';
        if duplicate || first_zero {
            not_exist = true;
            break;
        }
        result[*s] = Some(*c);
    }

    if not_exist {
        println!("-1");
    } else {
        println!(
            "{}",
            result
                .iter()
                .enumerate()
                .map(|(i, opt)| match opt {
                    Some(num_char) => num_char.to_string(),
                    None => {
                        if i == 0 && n > 1 {
                            String::from("1")
                        } else {
                            String::from("0")
                        }
                    }
                })
                .collect::<Vec<_>>()
                .concat()
        );
    }
}
