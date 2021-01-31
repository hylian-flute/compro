fn input_line() -> Vec<String> {
  let mut line = String::new();
  std::io::stdin().read_line(&mut line).unwrap();
  (&line[..]).trim_end().split(' ').map(|word| String::from(word)).collect()
}

fn main() {
  let n: usize = (&input_line()[0]).parse().unwrap();
  let a: Vec<u64> = input_line().iter().map(|s| s.parse().unwrap()).collect();

  let sum = {
    let mut sum = 0_u64;
    for v in a.iter() { sum += *v }
    sum
  };

  let mut left = 0_usize;
  let mut right = 0_usize;
  let mut current_sum = a[0];
  let mut best_diff = sum;
  loop {
    if current_sum <= sum / 2 {
      right = (right + 1) % n;
      current_sum += a[right];
    } else {
      current_sum -= a[left];
      left = left + 1;
      if left >= n { break; }
    }
    let diff = if current_sum <= sum / 2 {
      (sum - current_sum) - current_sum
    } else {
      current_sum - (sum - current_sum)
    };
    if diff < best_diff { best_diff = diff };
  }
  println!("{}", best_diff);
}
