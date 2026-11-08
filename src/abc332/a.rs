use proconio::input;

fn main() {
    input! {
        n: usize,
        s: i32,
        k: i32,
        pq: [(i32, i32); n],
    };

    let prize_sum = pq.iter().fold(0, |s, (p, q)| s + p * q);
    let answer = prize_sum + if prize_sum >= s { 0 } else { k };
    println!("{}", answer);
}
