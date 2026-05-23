use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };

    let mut id_and_win_count_list = Vec::<(usize, i64)>::new();
    for i in 0..n {
        let id = i + 1;
        id_and_win_count_list.push((id, 0));
    }

    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'o' {
                id_and_win_count_list[i].1 += 1;
            }
        }
    }

    id_and_win_count_list.sort_unstable_by(|a, b| {
        if a.1 != b.1 {
            b.1.cmp(&a.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    println!(
        "{}",
        id_and_win_count_list
            .iter()
            .map(|p| format!("{}", p.0))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
