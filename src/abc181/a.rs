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

fn main() {
  let mut input = Input::new();
  let n: u32 = input.get().parse().unwrap();
  let color = if n % 2 == 0 {
    "White"
  } else {
    "Black"
  };
  println!("{}", color);
}
