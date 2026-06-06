use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    let mut result = u32::MAX;
    for pattern in 0..(2_u32.pow(n as u32 - 1)) {
        let mut p = pattern;
        let mut current_xor = 0_u32;
        let mut current_or = 0_u32;
        for i in 0..n {
            if i == 0 {
                current_or = a[i];
                continue;
            }
            if p % 2 == 0 {
                current_or |= a[i];
            } else {
                current_xor ^= current_or;
                current_or = a[i];
            }
            p /= 2;
        }
        current_xor ^= current_or;
        result = result.min(current_xor);
    }

    println!("{}", result);
}
