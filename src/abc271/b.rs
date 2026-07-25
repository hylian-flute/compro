use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    };
    let mut l = vec![0_usize; n];
    let mut a = vec![Vec::<i32>::new(); n];
    for i in 0..n {
        input! {
            l_i: usize,
            a_i: [i32; l_i],
        };
        l[i] = l_i;
        a[i] = a_i;
    }

    for _ in 0..q {
        input! {
            s_i: Usize1,
            t_i: Usize1,
        };
        println!("{}", a[s_i][t_i]);
    }
}
