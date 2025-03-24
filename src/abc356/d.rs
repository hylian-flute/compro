// https://atcoder.jp/contests/abc356/tasks/abc356_d

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
  static MOD: usize = 998244353;

  let mut input = Input::new();
  let n: u64 = input.get().parse().unwrap();
  let m: u64 = input.get().parse().unwrap();

  let m_binary = format!("{:b}", m);
  let m_vec: Vec<char> = m_binary.chars().collect();
  let m_len = m_binary.len();

  let mut popcount: usize = m_vec.iter().filter(|x| **x == '1').count();
  if n >= m {
    let result = popcount % MOD;
    println!("{}", &popcount);
  }

  let mut current_m = m;
  for i in 0..m_len {
    popcount -= 1;

    let diff = 2_u64.pow((m_len - 1 - i) as u32);
    current_m -= diff;
    if n >= current_m {
      let result = popcount % MOD;
      println!("{}", &popcount);
      break;
    }
  }
}
