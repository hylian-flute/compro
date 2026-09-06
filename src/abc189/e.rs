use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
        m: usize,
    };

    let mut o_ux_uy = [(0_i64, 0_i64), (1, 0), (0, 1)];
    let mut history = vec![o_ux_uy.clone()];

    for _ in 0..m {
        input! {
            op_id: usize,
        };
        match op_id {
            1 => {
                o_ux_uy = o_ux_uy.map(|(x, y)| (y, -x));
            }
            2 => {
                o_ux_uy = o_ux_uy.map(|(x, y)| (-y, x));
            }
            3 => {
                input! {
                    p: i64,
                };
                o_ux_uy = o_ux_uy.map(|(x, y)| ((2 * p - x), y));
            }
            4 => {
                input! {
                    p: i64,
                };
                o_ux_uy = o_ux_uy.map(|(x, y)| (x, (2 * p - y)));
            }
            _ => panic!(),
        };
        history.push(o_ux_uy.clone());
    }

    input! {
        q: usize,
        ab: [(usize, Usize1); q],
    };

    for (a, b) in ab {
        let x = history[a][0].0
            + (history[a][1].0 - history[a][0].0) * xy[b].0
            + (history[a][2].0 - history[a][0].0) * xy[b].1;
        let y = history[a][0].1
            + (history[a][1].1 - history[a][0].1) * xy[b].0
            + (history[a][2].1 - history[a][0].1) * xy[b].1;
        println!("{} {}", x, y);
    }
}
