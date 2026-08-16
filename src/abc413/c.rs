// WA

use proconio::input;

fn main() {
    input! {
        q: usize,
    };

    let mut query_1_list = Vec::<(usize, i128)>::new();
    let mut query_2_list = Vec::<usize>::new();
    for _ in 0..q {
        input! {
            query_type: usize,
        };
        if query_type == 1 {
            input! {
                c: usize,
                x: i128,
            };
            query_1_list.push((c, x));
        } else {
            input! {
                k: usize,
            };
            query_2_list.push(k);
        }
    }

    let mut sum_required_idx_list = Vec::<usize>::new();
    {
        let mut current_idx = 0_usize;
        for k in query_2_list {
            current_idx += k;
            sum_required_idx_list.push(current_idx);
        }
    }

    let mut sum_list = Vec::<i128>::new();
    {
        let mut current_idx = 0_usize;
        let mut current_answer_idx = 0_usize;
        let mut current_sum = 0_i128;
        for (c, x) in query_1_list {
            let next_idx = current_idx + c;
            while current_answer_idx < sum_required_idx_list.len()
                && sum_required_idx_list[current_answer_idx] < next_idx
            {
                let idx_diff = sum_required_idx_list[current_answer_idx] - current_idx;
                sum_list.push(current_sum + x * idx_diff as i128);
                current_answer_idx += 1;
            }
            current_sum += x * c as i128;
            current_idx = next_idx;
        }
    }

    for pair in [vec![0], sum_list].concat().windows(2) {
        println!("{}", pair[1] - pair[0]);
    }
}
