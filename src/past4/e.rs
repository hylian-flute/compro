fn input_line() -> Vec<String> {
  let mut line = String::new();
  std::io::stdin().read_line(&mut line).unwrap();
  (&line[..]).trim_end().split(' ').map(|word| String::from(word)).collect()
}

fn main() {
  let n: usize = (&input_line()[0]).parse().unwrap();
  let mut s: Vec<char> = {
    let line = &input_line()[0];
    line.chars().collect()
  };

  let mut changed = false;
  'outer: for i in 0..(n - 1) {
    for j in (i + 1)..n {
      if s[i] != s[j] && i != n - j - 1 {
        let tmp = s[i];
        s[i] = s[j];
        s[j] = tmp;
        changed = true;
        break 'outer;
      }
    }
  }

  if changed {
    for i in 0..n {
      print!("{}", s[i]);
    }
    println!("");
  } else {
    println!("None");
  }
}