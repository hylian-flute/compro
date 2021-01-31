fn input_line() -> Vec<String> {
  let mut line = String::new();
  std::io::stdin().read_line(&mut line).unwrap();
  (&line[..]).trim_end().split(' ').map(|word| String::from(word)).collect()
}

fn main() {
  let (n, m): (usize, usize) = {
    let line = input_line();
    (line[0].parse().unwrap(), line[1].parse().unwrap())
  };

  let x: [u64; 3] = {
    let line = input_line();
    [
      line[0].parse().unwrap(),
      line[1].parse().unwrap(),
      line[2].parse().unwrap()
    ]
  };

  let s: Vec<usize> = {
    let line = &input_line()[0];
    line.chars().map(|c| match c {
      'A' => 0,
      'B' => 1,
      _ => 2,
    }).collect()
  };

  let mut paths = Vec::new();
  for _ in 0..n { paths.push(Vec::new()); }
  for _ in 0..m {
    let line = input_line();
    let (a, b): (usize, usize) =
      (line[0].parse().unwrap(), line[1].parse().unwrap());
    let c: u64 = line[2].parse().unwrap();
    paths[a - 1].push((b - 1, c));
    paths[b - 1].push((a - 1, c));
  }

  let mut costs = Vec::new();
  for _ in 0..n { costs.push(18446744073709551615_u64) }
  let mut queue = std::collections::VecDeque::new();
  queue.push_back((n - 1, 0_u64));
  while queue.len() > 0 {
    let start = queue.pop_front().unwrap();

    for i in 0..n {
      if (s[start.0] == 0 && s[i] == 1) || (s[start.0] == 1 && s[i] == 0) {
        if start.1 + x[0] < costs[i] {
          queue.push_back((i, start.1 + x[0]));
        }
      } else if (s[start.0] == 0 && s[i] == 2) || (s[start.0] == 2 && s[i] == 0) {
        if start.1 + x[1] < costs[i] {
          queue.push_back((i, start.1 + x[1]));
        }
      } else if (s[start.0] == 1 && s[i] == 2) || (s[start.0] == 2 && s[i] == 1) {
        if start.1 + x[2] < costs[i] {
          queue.push_back((i, start.1 + x[2]));
        }
      }
    }
    for path in paths[start.0].iter() {
      let cost = start.1 + path.1;
      if cost < costs[path.0] {
        queue.push_back((path.0, cost));
      }
    }
  }
  println!("{}", costs[0]);
}
