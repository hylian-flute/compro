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
  let mut s: Vec<Vec<bool>> = Vec::new();
  for _ in 0..n {
    s.push(input_line()[0].chars().map(|c| c == '#').collect());
  }

  let mut ans = 0_u64;
  for outer_y in 0..n {
    for outer_x in 0..m {
      if !s[outer_y][outer_x] {
        continue;
      }
      s[outer_y][outer_x] = false;

      // <main>
      let start = {
        let mut res = (0_usize, 0_usize);
        'decide_start: for y in 0..n {
          for x in 0..m {
            if !s[y][x] {
              res = (y, x);
              break 'decide_start;
            }
          }
        }
        res
      };

      let mut stack = Vec::new();
      stack.push(start);

      let mut checked = Vec::new();
      for y in 0..n {
        checked.push(Vec::new());
        for _ in 0..m {
          checked[y].push(false);
        }
      }

      while stack.len() > 0 {
        let point = stack.pop().unwrap();
        if s[point.0][point.1] || checked[point.0][point.1] {
          continue;
        };
        checked[point.0][point.1] = true;

        if point.0 >= 1 {
          stack.push((point.0 - 1, point.1));
        }
        if point.0 < n - 1 {
          stack.push((point.0 + 1, point.1));
        }
        if point.1 >= 1 {
          stack.push((point.0, point.1 - 1));
        }
        if point.1 < m - 1 {
          stack.push((point.0, point.1 + 1));
        }
      }

      let mut diff = false;
      'check: for y in 0..n {
        for x in 0..m {
          if s[y][x] == checked[y][x] {
            diff = true;
            break 'check;
          }
        }
      }

      if !diff {
        ans += 1;
      }
      // </main>

      s[outer_y][outer_x] = true;
    }
  }
  println!("{}", ans);
}
