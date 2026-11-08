use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    };

    let answer = a
        .iter()
        .all(|a_i| a_i % 2 == 1 || (a_i % 3 == 0 || a_i % 5 == 0));

    let answer = if answer { "APPROVED" } else { "DENIED" };
    println!("{}", answer);
}
