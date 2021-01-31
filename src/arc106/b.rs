fn input_line() -> Vec<String> {
  let mut line = String::new();
  std::io::stdin().read_line(&mut line).unwrap();
  (&line[..]).trim_end().split(' ').map(|word| String::from(word)).collect()
}

fn main() {
  let n: usize;
  let m: usize;
  {
    let line = input_line();
    n = (&line[0]).parse().unwrap();
    m = (&line[1]).parse().unwrap();
  }

  let a: Vec<i64> = input_line().iter().map(|s| { s.parse().unwrap() }).collect();
  let b: Vec<i64> = input_line().iter().map(|s| { s.parse().unwrap() }).collect();

  let mut paths: Vec<(usize, usize)> = Vec::new();
  for _ in 0..m {
    let line = input_line();
    let vertexes: (usize, usize) =
      ((&line[0]).parse().unwrap(), (&line[1]).parse().unwrap());
    paths.push((vertexes.0 - 1, vertexes.1 - 1));
  }

  let mut path_map: Vec<Vec<usize>> = Vec::new();
  for _ in 0..n { path_map.push(Vec::new()) }
  for path in paths.iter() {
    path_map[path.0].push(path.1);
    path_map[path.1].push(path.0);
  }

  let mut labels: Vec<usize> = Vec::new();
  for _ in 0..n { labels.push(0) }
  let mut label_definitions: Vec<bool> = Vec::new();
  for _ in 0..n { label_definitions.push(false) }

  let mut stack: Vec<(usize, usize)> = Vec::new();
  for i in 0..n {
    stack.push((i, i));
    loop {
      let tuple = match stack.pop() {
        Some(t) => t,
        None => break,
      };
      if label_definitions[tuple.0] { continue; }

      labels[tuple.0] = tuple.1;
      label_definitions[tuple.0] = true;
      for next in path_map[tuple.0].iter() {
        stack.push((*next, tuple.1));
      }
    }
  }

  let mut a_sums: Vec<i64> = Vec::new();
  let mut b_sums: Vec<i64> = Vec::new();
  for _ in 0..n {
    a_sums.push(0);
    b_sums.push(0);
  }
  for i in 0..n {
    a_sums[labels[i]] += a[i];
    b_sums[labels[i]] += b[i];
  }

  for i in 0..n {
    if a_sums[i] != b_sums[i] {
      println!("No");
      return;
    }
  }
  println!("Yes");
}
