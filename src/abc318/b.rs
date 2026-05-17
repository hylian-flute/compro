use proconio::input;

fn main() {
    input! {
        n: usize,
        abcd: [(usize, usize, usize, usize); n],
    };

    let mut blocks = vec![false; 100_usize.pow(2)];
    for (a, b, c, d) in abcd {
        for x in a..b {
            for y in c..d {
                blocks[y * 100 + x] = true;
            }
        }
    }

    let result = blocks.iter().filter(|bl| **bl).collect::<Vec<_>>().len();
    println!("{}", result);
}
