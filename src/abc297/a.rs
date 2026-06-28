use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i32,
        t: [i32; n],
    };

    let mut result = -1;
    for x in t.windows(2) {
        let (x1, x2) = (x[0], x[1]);
        if x2 - x1 <= d {
            result = x2; 
            break;
        }
    }
    println!("{}", result);
}
