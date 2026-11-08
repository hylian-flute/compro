use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: i64,
        a: [[i64; w]; h],
    };

    let mut costs_as_rb = vec![vec![0_i64; w]; h];
    for y in 0..h {
        for x in 0..w {
            let cost_on_u = if y > 0 {
                (costs_as_rb[y - 1][x]).min(a[y - 1][x]) + c
            } else {
                i64::MAX
            };
            let cost_on_l = if x > 0 {
                (costs_as_rb[y][x - 1]).min(a[y][x - 1]) + c
            } else {
                i64::MAX
            };
            costs_as_rb[y][x] = cost_on_u.min(cost_on_l);
        }
    }

    let mut costs_as_ru = vec![vec![0_i64; w]; h];
    for y in (0..h).rev() {
        for x in 0..w {
            let cost_on_b = if y < h - 1 {
                (costs_as_ru[y + 1][x]).min(a[y + 1][x]) + c
            } else {
                i64::MAX
            };
            let cost_on_l = if x > 0 {
                (costs_as_ru[y][x - 1]).min(a[y][x - 1]) + c
            } else {
                i64::MAX
            };
            costs_as_ru[y][x] = cost_on_b.min(cost_on_l);
        }
    }

    let mut answer = i64::MAX;
    for y in 0..h {
        for x in 0..w {
            let c = costs_as_rb[y][x].min(costs_as_ru[y][x]) + a[y][x];
            answer = answer.min(c);
        }
    }

    println!("{}", answer);
}
