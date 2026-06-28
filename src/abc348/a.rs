use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let result = (0..n)
        .map(|i| if i % 3 == 2 { 'x' } else { 'o' })
        .collect::<String>();
    println!("{}", &result);
}
