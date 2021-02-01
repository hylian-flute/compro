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

struct Point {
  x: i64,
  y: i64,
}

fn main() {
  let mut input = Input::new();
  let n: usize = input.get().parse().unwrap();
  let mut points = Vec::new();
  for _ in 0..n {
    points.push(Point {
      x: input.get().parse().unwrap(),
      y: input.get().parse().unwrap(),
    });
  }

  for i in 0..(n - 2) {
    for j in (i + 1)..(n - 1) {
      let x_diff_0 = points[j].x - points[i].x;
      let y_diff_0 = points[j].y - points[i].y;
      for k in (j + 1)..n {
        let x_diff_1 = points[k].x - points[i].x;
        let y_diff_1 = points[k].y - points[i].y;
        if y_diff_0 * x_diff_1 == y_diff_1 * x_diff_0 {
          println!("Yes");
          return;
        }
      }
    }
  }
  println!("No");
}
