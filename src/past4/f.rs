fn input_line() -> Vec<String> {
  let mut line = String::new();
  std::io::stdin().read_line(&mut line).unwrap();
  (&line[..]).trim_end().split(' ').map(|word| String::from(word)).collect()
}

fn main() {
  let (n, k): (usize, usize) = {
    let line = input_line();
    (line[0].parse().unwrap(), line[1].parse().unwrap())
  };
  let mut s = Vec::new();
  for _ in 0..n {
    s.push(input_line()[0].clone());
  }

  let mut word_times_map = std::collections::HashMap::new();
  for i in 0..n {
    let key: &str = &s[i];
    match word_times_map.get(key) {
      Some(times) => word_times_map.insert(key, times + 1),
      None => word_times_map.insert(key, 1_u64),
    };
  }

  let mut ranking = Vec::new();
  for (key, val) in word_times_map.iter() {
    ranking.push((key, val));
  }
  ranking.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

  let ans = ranking[k - 1];
  if
    (k >= 2 && ranking[k - 2].1 == ans.1)
    || (k < ranking.len() && ranking[k].1 == ans.1)
  {
    println!("AMBIGUOUS");
  } else {
    println!("{}", ans.0);
  };
}
