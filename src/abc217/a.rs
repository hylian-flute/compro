// https://atcoder.jp/contests/abc217/tasks/abc217_a

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
    let s: String = input.get().parse().unwrap();
    let t: String = input.get().parse().unwrap();

    let result = if s < t {
        String::from("Yes")
    } else {
        String::from("No")
    };

    println!("{}", &result);
}
