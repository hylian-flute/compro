use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        h: [i32; n],
        w: [i32; m],
    };

    let mut h = h;
    h.sort_unstable();

    let diffs = h.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();

    let mut sum_of_diffs_without_teacher_pair = Vec::<i32>::new();
    {
        let mut cur = 0_i32;
        for i in 0..(n / 2) {
            cur += diffs[1 + 2 * i];
        }
        for i in 0..n {
            sum_of_diffs_without_teacher_pair.push(cur);
            cur = if i % 2 == 0 {
                cur
            } else {
                cur + diffs[i - 1] - diffs[i]
            };
        }
        sum_of_diffs_without_teacher_pair.push(*sum_of_diffs_without_teacher_pair.last().unwrap());
    }

    let mut answer = i32::MAX;
    for w_i in w {
        let pp = h.partition_point(|&h_i| h_i < w_i);
        let diff = if pp % 2 == 0 {
            h[pp] - w_i
        } else {
            w_i - h[pp - 1]
        };
        let sum = diff + sum_of_diffs_without_teacher_pair[pp];
        if sum < answer {
            answer = sum;
        }
    }

    println!("{}", answer);
}
