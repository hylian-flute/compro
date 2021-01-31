fn input_line() -> Vec<String> {
  let mut line = String::new();
  std::io::stdin().read_line(&mut line).unwrap();
  (&line[..]).trim_end().split(' ').map(|word| String::from(word)).collect()
}

fn main() {
  let (n, m): (usize, usize) = {
    let line = input_line();
    (
      (&line[0]).parse().unwrap(),
      (&line[1]).parse().unwrap(),
    )
  };
  let mut s: Vec<String> = Vec::new();
  for _ in 0..n {
    s.push(input_line()[0].clone());
  }

  let mut ans: Vec<Vec<u64>> = Vec::new();
  for i in 0..n {
    ans.push(Vec::new());
    for _ in 0..m {
      ans[i].push(0);
    }
  }

  for y in 0..n {
    for x in 0..m {
      let u: usize = {
        if y > 0 {
          y - 1
        } else {
          y
        }
      };
      for py in u..=(y + 1) {
        if py >= n {
          continue;
        }
        let l: usize = {
          if x > 0 {
            x - 1
          } else {
            x
          }
        };
        for px in l..=(x + 1) {
          if px >= m {
            continue;
          }
          if &s[py][px..(px + 1)] == "#" {
            ans[y][x] += 1;
          }
        }
      }
    }
  }

  for y in 0..n {
    for x in 0..m {
      print!("{}", ans[y][x]);
    }
    println!("");
  }
}