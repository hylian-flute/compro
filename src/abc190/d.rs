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

// xが奇数かつNをxで割れる
// xが偶数かつNをxで割ると整数+0.5

fn main() {
  let mut input = Input::new();
  let n: u64 = input.get().parse().unwrap();

  let mut ans = 0_usize;
  for i in 1..(2*n) {
    if i * i > 2 * n {
      break;
    }
    if i * i <= n && n % i == 0 {
      let x = i;
      let y = n / i;
      if x % 2 == 1 {
        ans += 1;
      }
      if x != y && y % 2 == 1 {
        ans += 1;
      }
    }
    if (2 * n) % i == 0 {
      let x = i;
      let y = (2 * n) / i;
      if n % x != 0 && x % 2 == 0 {
        ans += 1;
      }
      if x != y && n % y != 0 && y % 2 == 0 {
        ans += 1;
      }
    }
  }
  println!("{}", ans);
}
