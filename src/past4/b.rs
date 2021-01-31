fn input_line() -> Vec<String> {
  let mut line = String::new();
  std::io::stdin().read_line(&mut line).unwrap();
  (&line[..]).trim_end().split(' ').map(|word| String::from(word)).collect()
}

fn main() {
  let (x, y): (f64, f64) = {
    let line = input_line();
    (
      (&line[0]).parse().unwrap(),
      (&line[1]).parse().unwrap(),
    )
  };

  if y == 0.0 {
    println!("ERROR");
    return;
  }

  let ans = x / y * 100.0;
  println!("{:.2}", ans.floor() / 100.0);
}