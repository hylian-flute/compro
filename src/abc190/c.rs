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

struct Condition {
  a: usize,
  b: usize,
}

struct Human {
  c: usize,
  d: usize,
}

fn main() {
  let mut input = Input::new();
  let n: usize = input.get().parse().unwrap();
  let m: usize = input.get().parse().unwrap();
  let mut conditions = Vec::new();
  for _ in 0..m {
    conditions.push(Condition {
      a: input.get().parse().unwrap(),
      b: input.get().parse().unwrap(),
    })
  }
  let k: usize = input.get().parse().unwrap();
  let mut humans = Vec::new();
  for _ in 0..k {
    humans.push(Human {
      c: input.get().parse().unwrap(),
      d: input.get().parse().unwrap(),
    })
  }

  let mut stack: Vec<Vec<usize>> = vec![Vec::new()];

  let mut result = 0_usize;
  while stack.len() > 0 {
    let dish_indexes = stack.pop().unwrap();
    if dish_indexes.len() >= k {
      let mut dishes = Vec::new();
      for _ in 0..n {
        dishes.push(false);
      }
      for i in dish_indexes {
        dishes[i] = true;
      }

      let mut true_condition_num = 0_usize;
      for condition in conditions.iter() {
        if dishes[condition.a - 1] && dishes[condition.b - 1] {
          true_condition_num += 1;
        }
      }
      if true_condition_num > result {
        result = true_condition_num;
      }
    } else {
      let mut x = dish_indexes.clone();
      x.push(humans[dish_indexes.len()].c - 1);
      stack.push(x);
      let mut y = dish_indexes.clone();
      y.push(humans[dish_indexes.len()].d - 1);
      stack.push(y);
    }
  }
  println!("{}", result);
}
