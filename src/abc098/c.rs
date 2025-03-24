// https://atcoder.jp/contests/abc098/tasks/arc098_a

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
  let n: usize = input.get().parse().unwrap();
  let s: String = input.get().parse().unwrap();

  let s_vec: Vec<char> = s.chars().collect();
  let mut initial_sum_cost = 0i64;
  let mut min_cost_diff = 0i64;
  let mut current_cost_diff = 0i64;
  for i in 1..n {
    let prev_char = s_vec.get(i - 1).unwrap();
    let current_char = s_vec.get(i).unwrap();

    if *current_char == 'E' {
        // E向きの人数 = 0番目がリーダーの時のコスト
        initial_sum_cost += 1;
    }

    current_cost_diff +=
        (if *prev_char == 'W' { 1 } else { 0 }) // 前のリーダーはW向きなら振り向く必要がある
        + (if *current_char == 'E' { -1 } else { 0 }); // 今のリーダーはE向きなら振り向く必要がなくなる
    if current_cost_diff < min_cost_diff {
        min_cost_diff = current_cost_diff;
    }
  }

  let result = initial_sum_cost + min_cost_diff;

  println!("{}", &result);
}

