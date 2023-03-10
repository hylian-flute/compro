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

// n を何通りの2つの正の整数の掛け算として表せるかを数える
fn count_multi_pattern(n: usize) -> usize {
  let mut pattern = 0usize;
  for l in 1..=((n as f64).sqrt() as usize) {
    if n % l == 0 {
      // l が丁度 n の平方根でなければ掛け算は可換なので倍のパターンが存在
      pattern += if l * l == n { 1 } else { 2 };
    }
  }
  pattern
}

pub fn main() {
  let mut input = Input::new();
  let n: usize = input.get().parse().unwrap();

  let mut ans = 0usize;
  for ab in 1..=((n + 1) / 2) {
    let cd = n - ab;
    let pattern_ab = count_multi_pattern(ab);
    let pattern_cd = count_multi_pattern(cd);

    let pattern = pattern_ab * pattern_cd;
    // `pattern` が `n / 2` 丁度でないなら、ABとCDを入れ替えた倍のパターンが存在。
    let pattern = pattern * if ab < n / 2 { 2 } else { 1 };
    ans += pattern;
  }
  println!("{}", &ans);
}
