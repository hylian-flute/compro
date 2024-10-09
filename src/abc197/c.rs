// https://atcoder.jp/contests/abc197/tasks/abc197_c

// パターンが2**19通りしかないから全探索で間に合う

struct Input {
    words: Vec<String>,
    p: usize,
}

impl Input {
    fn new() -> Self {
        Input {
            words: Vec::new(),
            p: 0,
        }
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

fn main() {
    let mut input = Input::new();
    let n: usize = input.get().parse().unwrap();
    let mut a = Vec::new();
    for _i in 0..n {
        let a_i: u32 = input.get().parse().unwrap();
        a.push(a_i);
    }
    let a = a;

    let mut result = 2u32.pow(30);
    for x in 0..2u32.pow((n as u32) - 1) {
        let mut or_results = vec![a[0]];
        for i in 1..n {
            let splited = (x >> (i - 1)) & 1;
            if splited == 1 {
                or_results.push(a[i]);
            } else {
                let last_idx = or_results.len() - 1;
                or_results[last_idx] |= a[i];
            }
        }

        let mut xor_result = or_results[0];
        for j in 1..or_results.len() {
            xor_result ^= or_results[j];
        }

        if xor_result < result {
            result = xor_result;
        }
    }

    println!("{}", &result);
}
