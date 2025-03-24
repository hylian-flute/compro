// https://atcoder.jp/contests/abc374/tasks/abc374_a

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
  let s: String = input.get().parse().unwrap();

  let result = if s.ends_with("san") {
    "Yes"
  } else {
    "No"
  };

  println!("{}", &result);
}
