use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        n: usize,
        cd: [(f64, f64); n],
    };

    let (a, b) = if a < b { (a, b) } else { (b, a) };

    let calc_theta = |c: f64| {
        let mut min = 0_f64;
        let mut max = std::f64::consts::PI / 4.0;

        for _ in 0..1000 {
            let theta = min + (max - min) / 2.0;
            if a * theta.cos() + b * theta.sin() < c {
                min = theta;
            } else {
                max = theta;
            }
        }

        min + (max - min) / 2.0
    };

    for (c, d) in cd {
        let (c, d) = if c < d { (c, d) } else { (d, c) };
        if a <= c && b <= d {
            println!("YES");
        } else if a > c {
            println!("NO");
        } else {
            let theta = calc_theta(c);
            let min_d = a * theta.sin() + b * theta.cos();
            println!("{}", if d >= min_d { "YES" } else { "NO" });
        }
    }
}
