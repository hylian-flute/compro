use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i32; n],
        x: [i32; q],
    };

    let mut a = a;
    a.sort_unstable();
    for x_i in x {
        let p = a.partition_point(|&a_i| a_i < x_i);
        println!("{}", n - p);
    }
}
