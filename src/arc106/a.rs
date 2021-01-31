fn input_line() -> Vec<String> {
  let mut line = String::new();
  std::io::stdin().read_line(&mut line).unwrap();
  (&line[..]).trim_end().split(' ').map(|word| String::from(word)).collect()
}

fn main() {
  let n: u64 = (&input_line()[0]).parse().unwrap();

  let mut a = 1_u32;
  loop {
    let first = 3_u64.pow(a);
    if first > n {
      println!("-1");
      return;
    }

    let mut b = 1_u32;
    loop {
      let sum = first + 5_u64.pow(b);
      if sum > n { break; }
      else if sum == n {
        println!("{} {}", a, b);
        return;
      }
      else { b += 1 };
    }
    a += 1;
  }
}
