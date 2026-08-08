use proconio::input;

fn is_triangle(p: &[(i64, i64); 3]) -> bool {
    let x_diff_0 = p[1].0 - p[0].0;
    let y_diff_0 = p[1].1 - p[0].1;
    let x_diff_1 = p[2].0 - p[1].0;
    let y_diff_1 = p[2].1 - p[1].1;

    if x_diff_0 == 0 || x_diff_1 == 0 {
        x_diff_0 != 0 || x_diff_1 != 0
    } else {
        y_diff_0 * x_diff_1 != y_diff_1 * x_diff_0
    }
}

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    };

    let mut answer = 0_usize;
    for i_0 in 0..(n - 2) {
        for i_1 in (i_0 + 1)..(n - 1) {
            for i_2 in (i_1 + 1)..n {
                if is_triangle(&[xy[i_0], xy[i_1], xy[i_2]]) {
                    answer += 1;
                }
            }
        }
    }

    println!("{}", answer);
}
