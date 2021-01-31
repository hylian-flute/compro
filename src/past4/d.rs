fn input_line() -> Vec<String> {
  let mut line = String::new();
  std::io::stdin().read_line(&mut line).unwrap();
  (&line[..]).trim_end().split(' ').map(|word| String::from(word)).collect()
}

fn main() {
  let n: usize = (&input_line()[0]).parse().unwrap();
  let mut s: Vec<u64> = Vec::new();
  {
    let line = &input_line()[0];
    for i in 0..n {
      s.push(match &line[i..(i + 1)] {
        "#" => 1,
        _ => 0,
      });
    };
  }

  let mut until_first_ninja = true;
  let mut left_space: u64 = 0;
  let mut current_space: u64 = 0;
  let mut largest_space: u64 = 0;

  for i in 0..n {
    if s[i] == 0 {
      current_space += 1;
      if until_first_ninja {
        left_space += 1;
      }
    } else {
      if current_space > largest_space {
        largest_space = current_space;
      }
      current_space = 0;
      until_first_ninja = false;
    }
  }

  let x = left_space;
  let mut y = current_space;
  if x + y < largest_space {
    y += largest_space - (x + y);
  }

  println!("{} {}", x, y);
}
