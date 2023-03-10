struct Input {
  words: std::collections::VecDeque<String>,
}

impl Input {
  fn new() -> Self {
    Input { words: std::collections::VecDeque::new() }
  }
  fn get(&mut self) -> String {
    if self.words.len() == 0 {
      let mut line = String::new();
      std::io::stdin().read_line(&mut line).unwrap();
      self.words = line.trim_end().split(' ').map(|word| String::from(word)).collect();
    };
    self.words.pop_front().unwrap()
  }
}

pub fn main() {
  let mut input = Input::new();
  let s: String = input.get().parse().unwrap();
  println!("{}", &s.to_uppercase());
}
