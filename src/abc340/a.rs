use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        d: i32,
    };

    let mut v = a;
    loop {
        println!("{}", v);
        if v == b {
            break;
        };
        v += d;
    }
}
