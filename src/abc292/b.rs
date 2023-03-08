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
  let n: usize = input.get().parse().unwrap();
  let q: i64 = input.get().parse().unwrap();
  let mut yellow_count = vec![0; n];
  for _ in 0..q {
    let c: i64 = input.get().parse().unwrap();
    let x: usize = input.get().parse().unwrap();
    if c == 1 {
      yellow_count[x - 1] += 1;
    } else if c == 2 {
      yellow_count[x - 1] += 2;
    }

    if c == 3 {
      let is_red = if yellow_count[x - 1] >= 2 {
        "Yes"
      } else {
        "No"
      };
      println!("{}", is_red);
    }
  }
}
