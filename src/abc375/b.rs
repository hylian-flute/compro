use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    };

    let xy = vec![xy, vec![(0, 0)]].concat();

    let mut cost = 0_f64;
    let mut current_xy = (0_i64, 0_i64);
    for (x, y) in xy {
        let (cx, cy) = current_xy;
        cost += (((cx - x).pow(2) + (cy - y).pow(2)) as f64).sqrt();
        current_xy = (x, y);
    }

    println!("{}", cost);
}
