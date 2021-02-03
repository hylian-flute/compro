struct Input {
  words: Vec<String>,
  p: usize,
}

impl Input {
  fn new() -> Self {
    Input { words: Vec::new(), p: 0 }
  }
  fn get(&mut self) -> &str {
    if self.words.len() <= self.p {
      let mut line = String::new();
      std::io::stdin().read_line(&mut line).unwrap();
      for word in line.trim_end().split(' ') {
        self.words.push(String::from(word));
      }
    };
    self.p += 1;
    &self.words[self.p - 1]
  }
}

struct Spell {
  x: u64,
  y: u64,
}

fn main() {
  let mut input = Input::new();
  let n: usize = input.get().parse().unwrap();
  let s: u64 = input.get().parse().unwrap();
  let d: u64 = input.get().parse().unwrap();

  let mut spells = Vec::new();
  for _ in 0..n {
    spells.push(Spell {
      x: input.get().parse().unwrap(),
      y: input.get().parse().unwrap(),
    })
  }

  for spell in spells.iter() {
    if spell.x < s && spell.y > d {
      println!("Yes");
      return;
    }
  }

  println!("No");
}
