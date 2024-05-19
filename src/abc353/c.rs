// https://atcoder.jp/contests/abc353/tasks/abc353_c

/*
 * f(x, y) を単に `x + y` と考えると、正数列の全ての数は `N - 1` 回足し合わされるので、
 * 解は `sum(A_1, A_2, ..., A_N) * (N - 1)` になる。
 * ある A_i に対して `A_i + A_j` が `10 ** 8` を超える `A_j` の個数は、
 * 正数列をあらかじめソートしておいて、二分探索で求めることができるため、
 * `O(N log N)` で解くことができる。
 */

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
  static M: i64 = 10_i64.pow(8);

  let mut input = Input::new();
  let n: usize = input.get().parse().unwrap();
  let mut a: Vec<i64> = Vec::new();
  for _ in 0..n {
    a.push(input.get().parse().unwrap());
  }

  a.sort_unstable();
  let mut over_m_count = 0_i64;
  for i in 0..(n - 1) {
    let p = (i + 1) + a[(i + 1)..].partition_point(|&a_i| a[i] + a_i < M);
    over_m_count += (n - p) as i64;
  }

  let mut ans = 0_i64;
  for a_i in a.iter() {
    ans += a_i * (n - 1) as i64;
  }
  ans -= M * over_m_count;

  println!("{}", &ans);
}
