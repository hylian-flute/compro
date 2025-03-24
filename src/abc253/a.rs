// https://atcoder.jp/contests/abc253/tasks/abc253_a

struct Input {
  words: Vec<String>,
  p: usize,
}

impl Input {
  fn new() -> Self {
    Input { words: Vec::new(), p: 0 }
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
  let a: i64 = input.get().parse().unwrap();
  let b: i64 = input.get().parse().unwrap();
  let c: i64 = input.get().parse().unwrap();

  let result = if (a > b && c > b) || (a < b && c < b) {
    "No"
  } else {
    "Yes"
  };

  println!("{}", &result);
}
