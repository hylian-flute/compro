use std::collections::{HashMap, HashSet};

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

struct Path {
  v1: usize,
  v2: usize,
}
impl Path {
  fn new(v1: usize, v2: usize) -> Self {
    if v1 < v2 {
      Path { v1, v2 }
    } else {
      Path { v1: v2, v2: v1 }
    }
  }
}

struct Count {
  v: usize,
  p: usize,
}

fn judge(n: &usize, m: &usize, p: &Vec<Path>) -> bool {
  if n != m {
    return false
  }

  // どの頂点がどの頂点と結ばれているかの列挙
  let mut connections = HashMap::new();
  for path in p.iter() {
    for i in 0..2 {
      let v1 = if i == 0 { &path.v1 } else { &path.v2 };
      let v2 = if i == 0 { &path.v2 } else { &path.v1 };
      if let None = connections.get(v1) {
        connections.insert(v1, HashSet::new());
      };
      let set = connections.get_mut(v1).unwrap();
      set.insert(v2);
    }
  }

  // 各頂点にそこから連結な最小の頂点を紐付ける
  let mut labels = HashMap::new();
  for v in 1..=(*n) {
    let label = v;
    let mut stack = vec![v];
    while stack.len() > 0 {
      let v = stack.pop().unwrap();
      if let None = labels.get(&v) {
        labels.insert(v, label);
        for c in connections.get(&v).unwrap().iter() {
          stack.push(**c);
        }
      }
    }
  }

  let mut counts = HashMap::new();
  for label in labels.iter() {
    if let None = counts.get(label.1) {
      counts.insert(*label.1, Count { v: 0, p: 0 });
    }
    let mut count = counts.get_mut(label.1).unwrap();
    count.v += 1;
  }
  for path in p.iter() {
    let label = labels.get(&path.v1).unwrap();
    if let None = counts.get(label) {
      counts.insert(*label, Count { v: 0, p: 0 });
    }
    let mut count = counts.get_mut(label).unwrap();
    count.p += 1;
  }

  let ret = counts.iter().all(|count| count.1.v == count.1.p);
  ret
}

pub fn main() {
  let mut input = Input::new();
  let n: usize = input.get().parse().unwrap();
  let m: usize = input.get().parse().unwrap();
  let mut p = Vec::new();
  for _ in 0..m {
    let v1: usize = input.get().parse().unwrap();
    let v2: usize = input.get().parse().unwrap();
    p.push(Path::new(v1, v2));
  }

  let ans = judge(&n, &m, &p);
  println!("{}", if ans { "Yes" } else { "No" });
}
