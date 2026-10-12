use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    };

    let mut nexts_from_vtx = vec![Vec::<usize>::new(); n];
    for (u, v) in uv {
        nexts_from_vtx[u].push(v);
        nexts_from_vtx[v].push(u);
    }

    let mut stack = vec![(0_usize, 0_usize)];
    let mut count = 1_usize;
    let mut visited_from_vtx = vec![false; n];
    visited_from_vtx[0] = true;
    while let Some(&(vtx, path_i)) = stack.last() {
        if let Some(&next_vtx) = nexts_from_vtx[vtx].get(path_i) {
            if visited_from_vtx[next_vtx] {
                stack.pop();
                stack.push((vtx, path_i + 1));
            } else {
                stack.push((next_vtx, 0));
                count += 1;
                if count >= 1000000 {
                    break;
                }
                visited_from_vtx[next_vtx] = true;
            }
        } else {
            stack.pop();
            visited_from_vtx[vtx] = false;
            if let Some((prev_vtx, prev_path_i)) = stack.pop() {
                stack.push((prev_vtx, prev_path_i + 1));
            }
        }
    }

    println!("{}", count);
}
