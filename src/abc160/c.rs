// https://atcoder.jp/contests/abc160/tasks/abc160_c

struct Input {
    words: Vec<String>,
    p: usize,
}

impl Input {
    fn new() -> Self {
        Input {
            words: Vec::new(),
            p: 0,
        }
    }
    fn get(&mut self) -> &str {
        if self.words.len() <= self.p {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            for word in line.trim_end().split(' ') {
                self.words.push(String::from(word));
            }
        };
        self.p += 1;
        &self.words[self.p - 1]
    }
}

fn main() {
    let mut input = Input::new();
    let k: i64 = input.get().parse().unwrap();
    let n: usize = input.get().parse().unwrap();
    let mut a = Vec::new();
    for _i in 0..n {
        let a_i: i64 = input.get().parse().unwrap();
        a.push(a_i);
    }
    let a = a;

    let mut max_distance = k - a[n - 1] + a[0];
    for i in 1..n {
        let distance = a[i] - a[i - 1];
        if distance > max_distance {
            max_distance = distance;
        };
    }
    let result = k - max_distance;

    println!("{}", &result);
}
