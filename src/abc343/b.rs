// https://atcoder.jp/contests/abc343/tasks/abc343_b

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
    let n: usize = input.get().parse().unwrap();
    let mut a = Vec::new();
    for _i in 0..n {
        let mut a_i = Vec::new();
        for _j in 0..n {
            let a_ij: i64 = input.get().parse().unwrap();
            a_i.push(a_ij == 1);
        }
        a.push(a_i);
    }
    let a = a;

    let mut lines = Vec::new();
    for i in 0..n {
        let mut row = Vec::new();
        for j in 0..n {
            if a[i][j] {
                row.push((j + 1).to_string());
            }
        }
        let line = row.join(" ");
        lines.push(line);
    }
    let result = lines.join("\n");

    println!("{}", &result);
}
