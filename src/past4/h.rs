fn input_line() -> Vec<String> {
  let mut line = String::new();
  std::io::stdin().read_line(&mut line).unwrap();
  (&line[..]).trim_end().split(' ').map(|word| String::from(word)).collect()
}

fn main() {
  let (n, m, k): (usize, usize, usize) = {
    let line = input_line();
    (
      line[0].parse().unwrap(),
      line[1].parse().unwrap(),
      line[2].parse().unwrap()
    )
  };

  let mut s: Vec<Vec<u32>> = Vec::new();
  for _ in 0..n {
    s.push(input_line()[0].chars().map(|c| c.to_digit(10).unwrap()).collect());
  }

  let check_l = |l: usize| {
    let mut ans = false;
    'outer: for above in 0..(n - l + 1) {
      for left in 0..(m - l + 1) {
        let mut map = std::collections::HashMap::new();
        for y in above..(above + l) {
          for x in left..(left + l) {
            match map.get(&s[y][x]) {
              Some(val) => map.insert(&s[y][x], val + 1),
              None => map.insert(&s[y][x], 1_usize),
            };
          }
        }
        let mut max_count = 0_usize;
        for val in map.values() {
          if val > &max_count {
            max_count = *val;
          }
        }
        if l * l - max_count <= k {
          ans = true;
          break 'outer;
        }
      }
    }
    ans
  };

  let mut ng = 0;
  let mut ok = std::cmp::min(n, m);
  if check_l(ok) {
    println!("{}", ok);
    return;
  }
  while ok - ng > 1 {
    let mid = (ng + ok) / 2;
    if !check_l(mid) {
      ok = mid;
    } else {
      ng = mid;
    }
  }

  println!("{}", ok - 1);
}
