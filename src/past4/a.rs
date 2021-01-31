fn input_line() -> Vec<String> {
  let mut line = String::new();
  std::io::stdin().read_line(&mut line).unwrap();
  (&line[..]).trim_end().split(' ').map(|word| String::from(word)).collect()
}

fn main() {
  let (a, b, c) = {
    let line = input_line();
    let res: (u64, u64, u64) = (
      (&line[0]).parse().unwrap(),
      (&line[1]).parse().unwrap(),
      (&line[2]).parse().unwrap(),
    );
    res
  };

  if (a > b && a < c) || (a < b && a > c) {
    println!("A");
  } else if (b > a && b < c) || (b < a && b > c) {
    println!("B");
  } else {
    println!("C");
  };
}